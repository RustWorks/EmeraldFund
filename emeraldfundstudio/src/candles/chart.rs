use crate::{
    node_editor::{
        node_trait::{
            CheapCloneNodeDataTypeWithValue, EFNodeFNSerialized, EFNodeFn, NodeDataTypeWithValue,
        },
        nodes::preview::PreviewNode,
    },
    node_runners::realtime::NODE_COMPUTE_CACHE,
    types::candles::generate_candles,
};
use chrono::{DateTime, Utc};
use ecolor::Color32;
use egui_plot::{BoxElem, BoxPlot, BoxSpread, Legend, Line, MarkerShape, Plot, PlotPoints, Points};
use egui_snarl::{InPinId, Snarl};
use epaint::Stroke;
use itertools::izip;
use polars::frame::DataFrame;

const CANDLE_RED: Color32 = Color32::from_rgb(255, 0, 0);
const CANDLE_GREEN: Color32 = Color32::from_rgb(0, 255, 0);

const MARKER_BUY: Color32 = Color32::from_rgb(12, 116, 169);
const MARKER_SELL: Color32 = Color32::from_rgb(163, 43, 138);

pub fn candles_to_box_chart(df: &DataFrame) -> Vec<BoxElem> {
    let opens = df.column("open").unwrap().f64().unwrap();
    let highs = df.column("high").unwrap().f64().unwrap();
    let lows = df.column("low").unwrap().f64().unwrap();
    let closes = df.column("close").unwrap().f64().unwrap();
    let volumes = df.column("volume").unwrap().f64().unwrap();
    let timestamps = df.column("timestamp").unwrap().u64().unwrap();

    izip!(
        opens.into_iter(),
        highs.into_iter(),
        lows.into_iter(),
        closes.into_iter(),
        volumes.into_iter(),
        timestamps.into_iter().enumerate()
    )
    .map(|(open, high, low, close, _volume, (idx, _timestamp))| {
        let open = open.unwrap();
        let high = high.unwrap();
        let low = low.unwrap();
        let close = close.unwrap();
        // let volume = volume.unwrap();
        let color = if close > open {
            CANDLE_GREEN
        } else {
            CANDLE_RED
        };
        BoxElem::new(
            idx as f64 * 0.01,
            BoxSpread::new(low, close, (close + open) / 2.0, open, high),
        )
        .box_width(0.007)
        .whisker_width(0.0)
        .fill(color)
        .stroke(Stroke::new(2.0, color))
    })
    .collect()
}

pub fn signals_as_markers<'a>(
    snarl: &Snarl<EFNodeFNSerialized<'_>>,
    box_chart: &[BoxElem],
) -> Vec<Points<'a>> {
    let mut result: Vec<Points<'a>> = Vec::new();
    snarl.node_ids().for_each(|(id, node)| {
        match node.get_node().get_name() {
            "ExecutePositionNode" => {
                // get input of this node, then traverse corresponding output id
                let in_pin = snarl.in_pin(InPinId { node: id, input: 0 });
                if in_pin.remotes.is_empty() {
                    return;
                }
                let outpin_id = in_pin.remotes.first().unwrap();
                if !NODE_COMPUTE_CACHE.contains_key(&outpin_id.node.0) {
                    return;
                }
                let cached_result = NODE_COMPUTE_CACHE.get(&outpin_id.node.0).unwrap();
                if cached_result.is_empty() {
                    return;
                }
                let cached_result = cached_result.first().unwrap();
                if let NodeDataTypeWithValue::Signal(signal) = &**cached_result {
                    let mut pt_sell = vec![];
                    let mut pt_buy = vec![];
                    signal.iter().enumerate().for_each(|(idx, signal)| {
                        match *signal {
                            -1 => {
                                pt_sell.push([(idx as f64) * 0.01, box_chart[idx].spread.median]);
                            }
                            1 => {
                                pt_buy.push([(idx as f64) * 0.01, box_chart[idx].spread.median]);
                            }
                            _ => {}
                        };
                    });
                    if !pt_sell.is_empty() {
                        result.push(
                            Points::new(pt_sell)
                                .name("Sell")
                                .color(MARKER_SELL)
                                .filled(true)
                                .radius(5.0)
                                .shape(MarkerShape::Down),
                        );
                    }
                    if !pt_buy.is_empty() {
                        result.push(
                            Points::new(pt_buy)
                                .name("Buy")
                                .color(MARKER_BUY)
                                .filled(true)
                                .radius(5.0)
                                .shape(MarkerShape::Up),
                        );
                    }
                }
            }
            _ => {}
        }
    });
    return result;
}

fn get_preview_outputs<'a>(
    snarl: &'a Snarl<EFNodeFNSerialized<'a>>,
) -> impl Iterator<Item = ([u8; 3], CheapCloneNodeDataTypeWithValue)> + use<'a> {
    snarl.node_ids().filter_map(|(id, node)| {
        if node.get_node().get_name() != "PreviewNode" {
            return None;
        }
        let in_pin = snarl.in_pin(InPinId { node: id, input: 0 });
        if in_pin.remotes.is_empty() {
            return None;
        }
        let idx_of_node_connected_to_preview = in_pin.remotes[0].node.0;
        if !NODE_COMPUTE_CACHE.contains_key(&idx_of_node_connected_to_preview) {
            return None;
        }
        let cache_of_node_connected_to_preview = NODE_COMPUTE_CACHE
            .get(&idx_of_node_connected_to_preview)
            .unwrap();
        let output_value = cache_of_node_connected_to_preview[in_pin.remotes[0].output].clone();
        let preview_node = node
            .get_node()
            .as_any()
            .downcast_ref::<PreviewNode>()
            .unwrap();
        return Some((preview_node.output_color, output_value));
    })
}

pub fn candlestick_chart(ui: &mut eframe::egui::Ui, snarl: &Snarl<EFNodeFNSerialized<'_>>) {
    let candles = generate_candles(21, 500).unwrap();
    let first_timestamp = candles
        .column("timestamp")
        .unwrap()
        .u64()
        .unwrap()
        .get(0)
        .unwrap() as u64;
    let box_chart = candles_to_box_chart(&candles);
    let markers = signals_as_markers(snarl, &box_chart);
    let data = BoxPlot::new(box_chart)
        // TODO: finish this formatter
        .element_formatter(Box::new(|elm, _| {
            format!(
                "High = {max:.decimals$}\
             \nOpen = {q3:.decimals$}\
             \nClose = {q1:.decimals$}\
             \nLow = {min:.decimals$}",
                max = elm.spread.upper_whisker,
                q3 = elm.spread.quartile3,
                q1 = elm.spread.quartile1,
                min = elm.spread.lower_whisker,
                decimals = 5
            )
        }));

    let plot = Plot::new("candlestick chart")
        .legend(Legend::default())
        .x_axis_formatter(|grid, _| {
            let d = (first_timestamp + ((grid.value * 100.0 * 60.0) as u64)) as i64;
            let datetime = DateTime::<Utc>::from_timestamp(d, 0).unwrap();
            datetime.format("%Y-%m-%d %H:%M").to_string()
        });
    ui.with_layout(
        eframe::egui::Layout::right_to_left(eframe::egui::Align::TOP),
        |ui| {
            plot.show(ui, |plot_ui| {
                plot_ui.box_plot(data);
                for marker in markers.into_iter() {
                    plot_ui.points(marker);
                }
                for output in get_preview_outputs(&snarl) {
                    if let NodeDataTypeWithValue::DecimalSequence(seq) = &*output.1 {
                        let f64_iter = seq.iter().enumerate().filter_map(|(i, x)| {
                            if let Some(x) = x {
                                return Some([i as f64 * 0.01, x]);
                            }
                            None
                        });
                        let line_points = PlotPoints::from_iter(f64_iter);
                        let line = Line::new(line_points)
                            .color(Color32::from_rgb(output.0[0], output.0[1], output.0[2]))
                            .style(egui_plot::LineStyle::Solid);
                        plot_ui.line(line);
                    }
                }
            });
        },
    );
}
