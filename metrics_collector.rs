//! 性能指标采集 - CPU、内存、TPS、节点监控
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct ChainMetrics {
    pub tps: f64,
    pub block_height: u64,
    pub peer_count: usize,
    pub mempool_size: usize,
    pub cpu_usage: f32,
    pub memory_usage_mb: u64,
    pub timestamp: u64,
}

pub struct MetricsCollector {
    metrics_history: Vec<ChainMetrics>,
    start_time: Instant,
    tx_total: u64,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            metrics_history: Vec::new(),
            start_time: Instant::now(),
            tx_total: 0,
        }
    }

    pub fn record(&mut self, metrics: ChainMetrics) {
        self.metrics_history.push(metrics);
        if self.metrics_history.len() > 100 {
            self.metrics_history.remove(0);
        }
    }

    pub fn update_tps(&mut self, new_txs: u64) -> f64 {
        self.tx_total += new_txs;
        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed == 0.0 { 0.0 } else { self.tx_total as f64 / elapsed }
    }

    pub fn get_average_tps(&self) -> f64 {
        let sum: f64 = self.metrics_history.iter().map(|m| m.tps).sum();
        if self.metrics_history.is_empty() { 0.0 } else { sum / self.metrics_history.len() as f64 }
    }

    pub fn export_metrics(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        if let Some(latest) = self.metrics_history.last() {
            map.insert("tps".to_string(), latest.tps.to_string());
            map.insert("height".to_string(), latest.block_height.to_string());
            map.insert("peers".to_string(), latest.peer_count.to_string());
            map.insert("cpu".to_string(), latest.cpu_usage.to_string());
        }
        map
    }
}
