use candle_core::Tensor;
use std::collections::HashMap;

/// Ситуатор для классификации латентных векторов
pub struct Situator {
    /// Хранилище известных сценариев (latents → scenario_id)
    scenarios: HashMap<Vec<f32>, u32>,
    /// Порог новизны для создания новых сценариев
    novelty_threshold: f32,
}

impl Situator {
    /// Инициализация ситуатора
    pub fn new(novelty_threshold: f32) -> Self {
        Situator {
            scenarios: HashMap::new(),
            novelty_threshold,
        }
    }

    /// Классификация нового латентного вектора
    pub fn classify(&mut self, z: &[f32]) -> u32 {
        // Поиск ближайшего известного сценария
        let mut min_dist = f32::INFINITY;
        let mut best_scenario = 0;

        for (scenario_latent, &scenario_id) in &self.scenarios {
            let dist = self.cosine_similarity(z, scenario_latent);
            if dist < min_dist {
                min_dist = dist;
                best_scenario = scenario_id;
            }
        }

        // Если расстояние больше порога, создаём новый сценарий
        if min_dist > self.novelty_threshold {
            let new_id = self.scenarios.len() as u32;
            self.scenarios.insert(z.to_vec(), new_id);
            new_id
        } else {
            best_scenario
        }
    }

    /// Косинусное расстояние между векторами
    fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f32 {
        let dot: f32 = a.iter().zip(b).map(|(&x, &y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot / (norm_a * norm_b)
        }
    }
}

/// Байесовский вывод с калибровкой уверенности
mod bayesian {
    use candle_core::Tensor;
    pub fn calibrate(logits: &Tensor, temperature: f32) -> Tensor {
        // Реализация температурного масштабирования
        logits.div_scalar(temperature)
    }
}
