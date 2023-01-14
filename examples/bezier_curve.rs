use std::{fs, process::Command};

use nalgebra::Vector2;
use niubi::{
    curve::{NonRationalCurve, ParametricCurve, RationalCurve},
    na::{BezierCurve, BsplineCurve, NurbsCurve},
};

fn main() {
    let curve = BezierCurve::new(vec![
        Vector2::new(0.0, 500.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(500.0, 0.0),
        Vector2::new(500.0, 500.0),
    ]);
    to_svg("bezier", &curve);

    let curve = BsplineCurve::new_uniform(
        vec![
            Vector2::new(0.0, 100.0),
            Vector2::new(0.0, 0.0),
            Vector2::new(500.0, 0.0),
            Vector2::new(500.0, 500.0),
        ],
        2,
    );
    to_bspline_svg("bspline", &curve);

    let curve = NurbsCurve::new(
        vec![
            (Vector2::new(500.0, 0.0), 1.0),
            (Vector2::new(0.0, 0.0), 1.0),
            (Vector2::new(0.0, 500.0), 2.0),
        ],
        vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        2,
    );
    to_nurbs_svg("nurbs", &curve);
}

fn to_svg(name: &str, curve: &BezierCurve<2>) {
    let mut lines = String::new();
    for i in 0..curve.control_points().len() - 1 {
        // curve.control_points()[i] curve.control_points()[i+1]
        lines += &format!(
            "<line x1='{}' y1='{}' x2='{}' y2='{}' stroke='orange' fill='transparent' stroke-width='1' />", 
            curve.control_points()[i].x + 10.0,
            curve.control_points()[i].y + 10.0,
            curve.control_points()[i+1].x + 10.0,
            curve.control_points()[i+1].y + 10.0,
        );
    }

    let mut polyline = String::new();
    let mut points = String::new();
    for d in 0..=100 {
        let p = curve.get_point((d as f64) / 100f64);
        points += &format!("{} {} ", p.x + 10.0, p.y + 10.0);
    }
    polyline += &format!("<polyline points='{points}' stroke='blue' fill='transparent' stroke-width='1' stroke-linejoin='round' />");

    let mut circles = String::new();
    for i in 0..curve.control_points().len() {
        // curve.control_points()[i] curve.control_points()[i+1]
        circles += &format!(
            r#"<circle cx="{}" cy="{}" r="2" stroke="red" fill="orange" stroke-width="5"/>"#,
            curve.control_points()[i].x + 10.0,
            curve.control_points()[i].y + 10.0,
        );
    }
    let file_string = format!(
        r#"
        <svg version="1.1" baseProfile="full" width="100vw" height="100vh" xmlns="http://www.w3.org/2000/svg">
            {lines}
            {polyline}
            {circles}
        </svg> 
        "#
    );
    let path = name.to_string() + ".svg";
    fs::write(&path, file_string).unwrap();
    Command::new("open").arg(&path).output().unwrap();
}

fn to_bspline_svg(name: &str, curve: &BsplineCurve<2>) {
    let mut lines = String::new();
    for i in 0..curve.control_points().len() - 1 {
        // curve.control_points()[i] curve.control_points()[i+1]
        lines += &format!(
            "<line x1='{}' y1='{}' x2='{}' y2='{}' stroke='orange' fill='transparent' stroke-width='1' />", 
            curve.control_points()[i].x + 10.0,
            curve.control_points()[i].y + 10.0,
            curve.control_points()[i+1].x + 10.0,
            curve.control_points()[i+1].y + 10.0,
        );
    }

    let mut polyline = String::new();
    let mut points = String::new();
    for d in 0..=100 {
        let p = curve.get_point((d as f64) / 100f64);
        points += &format!("{} {} ", p.x + 10.0, p.y + 10.0);
    }
    polyline += &format!("<polyline points='{points}' stroke='blue' fill='transparent' stroke-width='1' stroke-linejoin='round' />");

    let mut circles = String::new();
    for i in 0..curve.control_points().len() {
        // curve.control_points()[i] curve.control_points()[i+1]
        circles += &format!(
            r#"<circle cx="{}" cy="{}" r="2" stroke="red" fill="orange" stroke-width="5"/>"#,
            curve.control_points()[i].x + 10.0,
            curve.control_points()[i].y + 10.0,
        );
    }
    let file_string = format!(
        r#"
        <svg version="1.1" baseProfile="full" width="100vw" height="100vh" xmlns="http://www.w3.org/2000/svg">
            {lines}
            {polyline}
            {circles}
        </svg> 
        "#
    );
    let path = name.to_string() + ".svg";
    fs::write(&path, file_string).unwrap();
    Command::new("open").arg(&path).output().unwrap();
}

fn to_nurbs_svg(name: &str, curve: &NurbsCurve<2>) {
    let mut lines = String::new();
    for i in 0..curve.control_points().len() - 1 {
        // curve.control_points()[i] curve.control_points()[i+1]
        lines += &format!(
            "<line x1='{}' y1='{}' x2='{}' y2='{}' stroke='orange' fill='transparent' stroke-width='1' />", 
            curve.control_points()[i].to_control_point_and_weight().0.x + 10.0,
            curve.control_points()[i].to_control_point_and_weight().0.y + 10.0,
            curve.control_points()[i+1].to_control_point_and_weight().0.x + 10.0,
            curve.control_points()[i+1].to_control_point_and_weight().0.y + 10.0,
        );
    }

    let mut polyline = String::new();
    let mut points = String::new();
    for d in 0..=100 {
        let p = curve.get_point((d as f64) / 100f64);
        points += &format!("{} {} ", p.x + 10.0, p.y + 10.0);
    }
    polyline += &format!("<polyline points='{points}' stroke='blue' fill='transparent' stroke-width='1' stroke-linejoin='round' />");

    let mut circles = String::new();
    for i in 0..curve.control_points().len() {
        // curve.control_points()[i] curve.control_points()[i+1]
        circles += &format!(
            r#"<circle cx="{}" cy="{}" r="2" stroke="red" fill="orange" stroke-width="5"/>"#,
            curve.control_points()[i].to_control_point_and_weight().0.x + 10.0,
            curve.control_points()[i].to_control_point_and_weight().0.y + 10.0,
        );
    }
    let file_string = format!(
        r#"
        <svg version="1.1" baseProfile="full" width="100vw" height="100vh" xmlns="http://www.w3.org/2000/svg">
            {lines}
            {polyline}
            {circles}
        </svg> 
        "#
    );
    let path = name.to_string() + ".svg";
    fs::write(&path, file_string).unwrap();
    Command::new("open").arg(&path).output().unwrap();
}
