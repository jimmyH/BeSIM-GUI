use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Series {
    pub label: String,
    pub color: String,
    pub points: Vec<(f64, f64)>,
}

#[derive(Properties, PartialEq)]
pub struct LineChartProps {
    pub series: Vec<Series>,
}

fn build_polyline(points: &[(f64, f64)], min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> String {
    if points.is_empty() || (max_x - min_x).abs() < f64::EPSILON || (max_y - min_y).abs() < f64::EPSILON {
        return String::new();
    }

    let mut data = String::new();
    for (idx, (x, y)) in points.iter().enumerate() {
        let nx = ((*x - min_x) / (max_x - min_x)) * 100.0;
        let ny = 100.0 - ((*y - min_y) / (max_y - min_y)) * 100.0;
        if idx > 0 {
            data.push(' ');
        }
        data.push_str(&format!("{:.2},{:.2}", nx, ny));
    }
    data
}

#[function_component(LineChart)]
pub fn line_chart(props: &LineChartProps) -> Html {
    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for series in &props.series {
        for (x, y) in &series.points {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x);
            min_y = min_y.min(*y);
            max_y = max_y.max(*y);
        }
    }

    if !min_x.is_finite() || !max_x.is_finite() {
        min_x = 0.0;
        max_x = 1.0;
    }
    if !min_y.is_finite() || !max_y.is_finite() || (max_y - min_y).abs() < f64::EPSILON {
        min_y = 0.0;
        max_y = 1.0;
    }

    let polylines = props.series.iter().map(|series| {
        let data = build_polyline(&series.points, min_x, max_x, min_y, max_y);
        html! {
            <polyline class="chart-line" stroke={series.color.clone()} points={data} />
        }
    });

    let legend = props.series.iter().map(|series| {
        html! {
            <div class="chart-legend-item">
                <span class="chart-legend-swatch" style={format!("background: {}", series.color)}></span>
                <span>{series.label.clone()}</span>
            </div>
        }
    });

    html! {
        <div class="line-chart">
            <svg viewBox="0 0 100 100" preserveAspectRatio="none">
                <rect x="0" y="0" width="100" height="100" class="chart-bg" />
                { for polylines }
            </svg>
            <div class="chart-legend">
                { for legend }
            </div>
        </div>
    }
}
