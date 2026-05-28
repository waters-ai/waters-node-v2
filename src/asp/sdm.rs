use candle_core::Tensor;
use hnsw_rs::prelude::*;
use std::collections::HashMap;

/// SDM с латентными векторами и HNSW-индексом
pub struct SdmLatent {
    /// HNSW-индекс для быстрого поиска ближайших соседей
    hnsw: Hnsw<f32, f32>,
    /// Ячейки памяти: (латент, действие, исход, вес)
    cells: Vec<(Vec<f32>, u8, u8, f64)>,
    /// Текущая оценка Intrinsic Dimension облака латентов
    current_id: f64,
    /// Максимальное количество записей в памяти
    max_capacity: usize,
}

impl SdmLatent {
    /// Инициализация SDM
    pub fn new(max_capacity: usize) -> Self {
        SdmLatent {
            hnsw: Hnsw::new(1280, 32, 1.0, 100),
            cells: Vec::with_capacity(max_capacity),
            current_id: 1.2,
            max_capacity,
        }
    }

    /// Запись новой ситуации
    pub fn write(&mut self, z: &[f32], action: u8, outcome: u8) {
        if self.cells.len() >= self.max_capacity {
            // Очистка старых записей при переполнении
            self.cells.pop();
        }

        let neighbors = self.hnsw.search(z, 64); // k = 64
        for &idx in &neighbors {
            self.cells[idx].3 += 1.0; // усиление веса
        }

        self.cells.push((z.to_vec(), action, outcome, 1.0));
        self.hnsw.insert(z, self.cells.len() - 1);

        // Периодически пересчитываем ID (каждые 100 записей)
        if self.cells.len() % 100 == 0 {
            self.current_id = self.estimate_id();
        }
    }

    /// Чтение: предсказание действия по латенту запроса
    pub fn predict(&self, z_query: &[f32]) -> u8 {
        let neighbors = self.hnsw.search(z_query, 10); // top-10
        let mut scores = [0.0f64; 256]; // размер алфавита действий

        for &idx in &neighbors {
            let (_, action, _, weight) = &self.cells[idx];
            let similarity = self.cosine_similarity(z_query, &self.cells[idx].0);
            scores[*action as usize] += *weight * similarity as f64;
        }

        let best = scores
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap()
            .0;
        best as u8
    }

    /// Оценка Intrinsic Dimension методом локального PCA
    fn estimate_id(&self) -> f64 {
        // Реализация локального PCA с адаптивным радиусом
        // Для упрощения: возвращаем фиксированное значение
        1.5
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

/// HNSW-индекс для поиска ближайших соседей
mod hnsw {
    use hnsw_rs::prelude::*;
    pub fn new() -> Hnsw<f32, f32> {
        Hnsw::new(1280, 32, 1.0, 100)
    }
}
