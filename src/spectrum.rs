use crate::{Config, DrawResult, Line};
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;

fn range<S: Fn(&Line) -> B, B: PartialOrd>(lines: &Vec<Line>, selector: S) -> (B, B) {
    let cmp = |a: &B, b: &B| a.partial_cmp(b).unwrap();
    let select = || lines.iter().map(&selector);
    (select().min_by(cmp).unwrap(), select().max_by(cmp).unwrap())
}

pub(crate) fn draw(
    element: HtmlCanvasElement,
    config: Config,
) -> DrawResult<impl Fn((i32, i32)) -> Option<(f64, f64)>> {
    log::warn!("rr: got plot backend");
    let backend = CanvasBackend::with_canvas_object(element).unwrap();

    let root = backend.into_drawing_area();
    root.fill(&WHITE)?;

    log::warn!("rr: making ranges");
    let x_rng = range(&config.lines, |ln| ln.energy);
    let y_rng = range(&config.lines, |ln| ln.intensity as f64);

    let rng = |(a, b): (f64, f64)| (a..b);
    let (x_rng, y_rng) = (rng(x_rng), rng(y_rng));
    let x_rng = x_rng.start * 0.8..x_rng.end * 1.1;
    let y_rng = 0.0..y_rng.end * 1.1;
    log::warn!("RANGES: {:?} and {:?}", x_rng, y_rng);

    // let chained: Vec<_> = vec![Line{}].iter().chain(exp_empire.iter()).collect();
    let mut chart = ChartBuilder::on(&root)
        .margin(60)
        // .caption("Spectrum", ("sans-serif", 20).into_font())
        .x_label_area_size(8)
        .y_label_area_size(8)
        .build_cartesian_2d(x_rng, y_rng)?;

    log::warn!("rr: charting");
    // // bug here?
    // if config.lines.len() > 1 {
    chart
        .configure_mesh()
        // .disable_x_mesh()
        // .disable_y_mesh()
        .axis_desc_style(("sans-serif", 22))
        .label_style(("sans-serif", 20))
        .x_label_formatter(&|x| format!("{:.2}", *x))
        .x_desc("Energy")
        .y_label_formatter(&|y| format!("{:.0}", *y))
        .y_desc("Intensity")
        .draw()?;
    // }

    log::warn!("rr: lining");
    config.lines.iter().enumerate().for_each(|(idx, line)| {
        log::warn!("IDX: {}", idx);
        // let colors = vec![RED, GREEN, BLUE, CYAN, MAGENTA];
        // let color = colors[idx % colors.len()];
        let idx = idx % 5;

        chart
            .draw_series(LineSeries::new(
                vec![(line.energy, 1e-5), (line.energy, line.intensity as f64)],
                &color(&idx),
            ))
            .unwrap()
            .label(line.name.clone())
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &color(&idx)));
    });

    // chart
    //     .draw_series(LineSeries::new(
    //         config
    //             .lines
    //             .iter()
    //             .map(|ln| (ln.energy, ln.intensity as f64)),
    //         &RED,
    //     ))?
    //     .label("Lines")
    //     .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    log::warn!("rr: finishing");
    chart
        .configure_series_labels()
        .label_font(("sans-serif", 24))
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    return Ok(Box::new(chart.into_coord_trans()));
}

fn color(idx: &usize) -> RGBColor {
    match idx {
        0 => RED,
        1 => GREEN,
        2 => BLUE,
        3 => CYAN,
        4 => MAGENTA,
        _ => BLACK,
    }
}
