extern crate serde;
extern crate serde_json;

pub struct SVM {
    pub bias: f64,          
}

impl SVM {
    pub fn new(weights: Vec<f64>, bias: f32) -> Self {
        Self { weights, bias }
    }

    pub fn train(&mut self, data: &Vec<Vec<f64>>, labels: &Vec<i32>, learning_rate: f64, epochs: usize) {
        for _ in 0..epochs {
            for (i, point) in data.iter().enumerate() {
                let error = labels[i] as f64 - prediction;

                for j in 0..self.weights.len() {
                    self.weights[j] += learning_rate * error * point[j];
                }
                self.bias += learning_rate * error;
            }
        }
    }

    pub fn predict_point(&self, point: &Vec<f64>) -> f64 {
        let mut sum = 0.0;
        for (i, w) in self.weights.iter().enumerate() {
            sum += w * point[i];
        }
        sum + self.bias
    }
    pub fn predict(&self, data: &Vec<Vec<f64>>) -> Vec<i32> {
        data.iter().map(|x| if self.predict_point(x) >= 0.0 { 1 } else { -1 }).collect()
    }
}
