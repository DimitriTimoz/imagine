use std::{process::Command, path::Path};

use druid::{Data, Rect, Point, im::Vector};


#[derive(Debug, Clone, Data)]
pub struct OcrTextBox {
    pub boxes: Vector<Point>,
    pub text: String,
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
        let boxes = points_str.split('-').map(|point| {
            let point = point.trim_start_matches('[').trim_end_matches(']').replace(' ', "");
            let point: Vec<&str> = point.split(',').collect();

            if point.len() != 2 {
                panic!("Invalid point format");
            }
            
            Point::new(
                point[0].parse::<i32>().unwrap() as f64,
                point[1].parse::<i32>().unwrap() as f64,
            )
        });

        // Parse text
        let text = parts[1].to_string();

        // Parse confidence
        let confidence = parts[2].parse::<f64>().unwrap();
    
        Ok(Self {
            boxes: boxes.collect(),
            text,
            confidence,
        })
    }
}

#[derive(Clone, Data)]
pub struct Ocr {
    img_path: String,
    pub content: Vector<OcrTextBox>,
}

impl Ocr {
    pub fn get_text(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_str().unwrap();
        let output = Command::new("python3")
            .arg("python/get_text.py")
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
            content: content.into(),
        }
    }
}