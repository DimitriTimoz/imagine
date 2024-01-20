use std::{process::Command, path::Path};

#[derive(Debug)]
pub struct Point(i32, i32);

#[derive(Debug)]
pub struct OcrTextBox {
    boxes: Vec<(Point, Point)>,
    text: String,
    confidence: f64,
}

impl OcrTextBox {
    fn parse_ocr_text_box(input: &str) -> Result<OcrTextBox, String> {
        let parts: Vec<&str> = input.split(';').collect();
        if parts.len() != 3 {
            return Err("Invalid input format".to_string());
        }

        // Parse points
        // Remove ( and )
        let points_str = parts[0].trim_start_matches('(').trim_end_matches(')');
        let boxes: Vec<(Point, Point)> = points_str.split("),(").map(|point| {
            let points: Vec<&str> = point.split('-').map(|x| x.trim_start_matches('[').trim_end_matches(']')).collect();
            if points.len() != 2 {
                panic!("Invalid point format");
            }
            // Remove spaces everywhere
            let points: Vec<String> = points.iter().map(|x| x.replace(' ', "")).collect();

            let point1: Vec<&str> = points[0].split(',').collect();
            let point2: Vec<&str> = points[1].split(',').collect();

            if point1.len() != 2 || point2.len() != 2 {
                panic!("Invalid point format");
            }
            

            let point1 = Point(point1[0].parse::<i32>().unwrap(), point1[1].parse::<i32>().unwrap());
            let point2 = Point(point2[0].parse::<i32>().unwrap(), point2[1].parse::<i32>().unwrap());
            (point1, point2)

        }).collect();

        // Parse text
        let text = parts[1].to_string();

        // Parse confidence
        let confidence = parts[2].parse::<f64>().unwrap();
    
        Ok(OcrTextBox { boxes, text, confidence })
    }
    }

pub struct Ocr {
    img_path: String,
    pub content: Vec<OcrTextBox>,
}

impl Ocr {
    pub fn get_text(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_str().unwrap();
        let output = Command::new("python3")
            .arg("get_text.py")
            .arg(path)
            .output()
            .expect("Échec de l'exécution du script Python");
        let binding = output.stdout.clone();
        let binding = String::from_utf8_lossy(&binding);
        let rows = binding.lines();
        let mut content = Vec::new();
        for row in rows {
            content.push(OcrTextBox::parse_ocr_text_box(row).unwrap());
        }
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        Self {
            img_path: path.to_string(),
            content: Vec::new(),
        }
    }
}