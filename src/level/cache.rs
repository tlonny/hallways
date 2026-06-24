use std::collections::{HashSet, VecDeque};
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use lru::LruCache;
use url::Url;

use crate::util;

use super::{Level, LoadError};

const LEVEL_CACHE_CAPACITY: usize = 16;

#[derive(Clone)]
pub enum CacheEntry {
    Ready(Arc<Level>),
    Failed(Arc<LoadError>),
}

pub struct Cache {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    cache: LruCache<Url, CacheEntry>,
    work_queue: VecDeque<Url>,
    pending: HashSet<Url>,
    join_handle: Option<JoinHandle<Result<Level, LoadError>>>,
}

impl Cache {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        Self {
            device,
            queue,
            cache: LruCache::new(NonZeroUsize::new(LEVEL_CACHE_CAPACITY).unwrap()),
            work_queue: VecDeque::new(),
            pending: HashSet::new(),
            join_handle: None,
        }
    }

    pub fn get(&mut self, url: &Url) -> Option<CacheEntry> {
        if let Some(entry) = self.cache.get(url) {
            return Some(entry.clone());
        }

        if self.pending.contains(url) {
            return None;
        }

        self.pending.insert(url.clone());
        self.work_queue.push_back(url.clone());
        return None;
    }

    pub fn update(&mut self) {
        if let Some(join_handle) = self.join_handle.as_ref() {
            if !join_handle.is_finished() {
                return;
            }
        }

        if let Some(join_handle) = self.join_handle.take() {
            let url = self.work_queue.pop_front().unwrap();
            self.pending.remove(&url);
            let entry = match join_handle.join().unwrap() {
                Ok(level) => {
                    let message = format!("Successfully loaded level: {url}");
                    util::log(util::log::Level::Info, message);
                    CacheEntry::Ready(Arc::new(level))
                }
                Err(error) => {
                    util::log(
                        util::log::Level::Error,
                        format!("Failed to load level: {url}. Error: {error}"),
                    );
                    CacheEntry::Failed(Arc::new(error))
                }
            };

            if let Some((evicted_url, _)) = self.cache.push(url, entry) {
                util::log(
                    util::log::Level::Debug,
                    format!("Evicted level: {evicted_url}"),
                );
            }
        }

        let Some(url) = self.work_queue.front() else {
            return;
        };

        let device = Arc::clone(&self.device);
        let queue = Arc::clone(&self.queue);
        let url = url.clone();
        util::log(util::log::Level::Info, format!("Loading level: {url}"));
        self.join_handle = Some(thread::spawn(move || Level::load(url, &device, &queue)));
    }
}
