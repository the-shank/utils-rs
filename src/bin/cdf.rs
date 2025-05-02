use color_eyre::eyre::Result;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;

/// reads input from stdin
/// writes output to stdout
fn main() -> Result<()> {
    let stdin = io::stdin();
    let rdr = BufReader::new(stdin);

    // read the data
    let mut data = Vec::new();
    for line in rdr.lines() {
        let line = line?;
        let val = line.parse::<f64>()?;
        data.push(val);
    }
    data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    // calculate frequency distribution
    let markers: &[f64] = &[
        0.00, 0.05, 0.10, 0.15, 0.20, 0.25, 0.30, 0.35, 0.40, 0.45, 0.50, 0.55, 0.60, 0.65, 0.70,
        0.75, 0.80, 0.85, 0.90, 0.95, 1.00,
    ];

    // calculate the marker values
    let min_val = data[0];
    let max_val = data
        .last()
        .expect("atleast one value needs to be provided to calculate the stats");
    let mut marker_vals: Vec<f64> = Vec::with_capacity(markers.len());

    // identify the marker-vals corresponding to each marker
    for m in markers {
        let target = min_val + m * (max_val - min_val);
        let m_idx = match data.binary_search_by(|probe| probe.partial_cmp(&target).unwrap()) {
            Ok(idx) | Err(idx) => idx,
        };
        marker_vals.push(data[m_idx]);
    }

    // now we just count the values that are less that each marker-val
    let mut cdf_cnts: Vec<f64> = Vec::with_capacity(markers.len());
    for mv in marker_vals {
        let cnt = data.iter().take_while(|val| **val <= mv).count();
        #[allow(clippy::cast_precision_loss)]
        let percent = cnt as f64 / data.len() as f64;
        cdf_cnts.push(percent);
    }

    // output
    let mut w = BufWriter::new(io::stdout());
    for (marker, percent) in markers.iter().zip(cdf_cnts.iter()) {
        writeln!(&mut w, "{marker},{percent}")?;
    }

    Ok(())
}
