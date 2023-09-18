use csv::{ReaderBuilder, Writer, Trim};
use std::io::Read;
use std::io::Write;
use std::fs::File;
fn main() {
    // convert_to_polar();
    convert_to_car();
    // convert_to_polar_html();
    // convert_to_cartesian_html();
}
#[derive(Debug, PartialEq)]
#[derive(Clone)]
struct Point {
    x: f64,
    y: f64,
}
#[derive(Debug, PartialEq)]
#[derive(Clone)]
struct PolarPoint {
    r: f64,
    t: f64,
}

#[allow(dead_code)]
fn to_polar(pt_list: Vec<Point>) -> Vec<PolarPoint>{
    let mut result = Vec::new();
    for point in pt_list{
        let r = (point.x.powi(2) + point.y.powi(2)).sqrt();
        let t = (((point.x / point.y).atan()).to_degrees()).ceil();
        result.push(PolarPoint{
            r: r,
            t: t,
        })
    }
    result
}

#[test]
fn test_to_polar() {
    let input_points = vec![
        Point { x: 0.5, y: 0.5},
        Point { x: 1.5, y: 1.5}
    ];

    let to_polar = to_polar(input_points);
    let expect = vec![PolarPoint { r: 0.7071067811865476, t: 45.0},
    PolarPoint { r: 2.1213203435596424, t: 45.0}
];
    assert_eq!(to_polar, expect);
}

#[allow(dead_code)]
fn to_cartesian(pt_list: Vec<PolarPoint>) -> Vec<Point> {
    let mut result = Vec::new();
    for point in pt_list{
        let x = (point.r) * point.t.to_radians().cos();
        let y = (point.r) * point.t.to_radians().sin();
        result.push(Point{
            x: x,
            y: y
        })
    }
    result
}

#[test]
fn test_to_cartesian() {
    let input_points = vec![
        PolarPoint { r: 0.7071067811865476, t: 45.0},
        PolarPoint { r: 2.121320343559642, t: 45.0}
    ];

    let to_cartesian = to_cartesian(input_points);
    let expect = vec![Point { x: 0.5000000000000001, y: 0.5},
    Point { x: 1.4999999999999996, y: 1.4999999999999993}
];
    assert_eq!(to_cartesian, expect);
}

#[allow(dead_code)]
fn load_points_polar<R: Read>(rdr: R) -> Vec<PolarPoint> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .trim(Trim::All)
        .from_reader(rdr);
    let mut out_list = vec![];
        for record in reader.records() {
            if let Ok(rec) = record {
                let r: f64 = rec[0].parse().unwrap();
                let t: f64 = rec[1].parse().unwrap();
                out_list.push(PolarPoint { r: r, t: t });
                }
        }
    out_list
}

#[allow(dead_code)]
fn load_points_car<R: Read>(rdr: R) -> Vec<Point> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .trim(Trim::All)
        .from_reader(rdr);
    let mut out_list = vec![];
        for record in reader.records() {
            if let Ok(rec) = record {
                let x: f64 = rec[0].parse().unwrap();
                let y: f64 = rec[1].parse().unwrap();
                out_list.push(Point { x: x, y: y});
                }
        }
    out_list
}

#[allow(dead_code)]
fn save_points_polar<W: Write>(writer: W, pt_list: Vec<PolarPoint>) {
    let mut wtr = Writer::from_writer(writer);
    for point in pt_list{
        wtr.write_record(&[point.r.to_string(), point.t.to_string()]).unwrap();
    }
    wtr.flush().unwrap();
}

#[allow(dead_code)]
fn save_points_car<W: Write>(writer: W, pt_list: Vec<Point>) {
    let mut wtr = Writer::from_writer(writer);
    for point in pt_list{
        wtr.write_record(&[point.x.to_string(), point.y.to_string()]).unwrap();
    }
    wtr.flush().unwrap();
}

#[allow(dead_code)]
fn convert_to_polar() {
    let point = load_points_car(File::open("input.csv").unwrap());
    let to_polar = to_polar(point);
    let result = save_points_polar(File::create("output.csv").unwrap(), to_polar);
    result
}

#[allow(dead_code)]
fn convert_to_car() {
    let point = load_points_polar(File::open("input2.csv").unwrap());
    let to_car = to_cartesian(point);
    let result = save_points_car(File::create("output2.csv").unwrap(), to_car);
    result

}

#[allow(dead_code)]
fn convert_to_polar_html() {
    let mut table = String::new();
    table.push_str("<!DOCTYPE html>
<html>
    <head>
        <title>Cartesian to Polar</title>
        <style> table, th, td {
            border: 1px solid #000000;
            text-align: center;
            width: 50%;
            border-collapse: collapse; 
            }
        </style>
        <h1>Cartesian to Polar</h1>
    </head>
    <body>
        <table>
            <thead>
                <tr>
                    <th>Cartesian</th>
                    <th>Polar</th>
                </tr>
            </thead>
            <tbody>");

    let point = load_points_car(File::open("input.csv").unwrap());
    let to_polar = to_polar(point.clone());

    for (car,polar) in point.iter().zip(to_polar.iter()){
        table.push_str(&format!("<tr><td>({:.1}, {:.1})</td><td>({:.1}, {:.1})</td></tr>", car.x, car.y, polar.r, polar.t));
    }
    
    table.push_str("</tbody>
                        </table>
                    </body>
                </html>"
            );

    let mut file = File::create("car_to_polar.html").expect("none");
    file.write(table.as_bytes()).expect("none");

    println!("Created a file car_to_polar.html")
    
}

#[allow(dead_code)]
fn convert_to_cartesian_html() {
    let mut table = String::new();
    table.push_str("<!DOCTYPE html>
<html>
    <head>
        <title>Polar to Cartesian</title>
        <style> table, th, td {
            border: 1px solid #000000;
            text-align: center;
            width: 50%;
            border-collapse: collapse; 
            }
        </style>
        <h1>Polar to Cartesian</h1>
    </head>
    <body>
        <table>
            <thead>
                <tr>
                    <th>Polar</th>
                    <th>Cartesian</th>
                </tr>
            </thead>
            <tbody>");  

    let point = load_points_polar(File::open("input2.csv").unwrap());
    let to_cartesian = to_cartesian(point.clone());

    for (polar,car) in point.iter().zip(to_cartesian.iter()){
        table.push_str(&format!("<tr><td>({:.1}, {:.1})</td><td>({:.1}, {:.1})</td></tr>", polar.r, polar.t, car.x, car.y));
    }
    
    table.push_str("</tbody>
                        </table>
                    </body>
                </html>"
            );
    let mut file = File::create("polar_to_car.html").expect("none");
    file.write(table.as_bytes()).expect("none");

    println!("Created a file polar_to_car.html")
}
