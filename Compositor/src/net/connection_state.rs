use std::time::Instant;
pub struct ConnectionStats {
    connected_at: Instant,
    last_activity: Instant,
    error_count: u32,
    recovery_attempts: u32,
}

impl ConnectionStats {
    pub fn new(
        connected_at: Instant,
        last_activity: Instant,
        error_count: u32,
        recovery_attempts: u32,
    ) -> Self {
        Self {
            connected_at,
            last_activity,
            error_count,
            recovery_attempts,
        }
    }
    
    pub fn error_rest(&mut self) {
        self.error_count = 0;
    }
    
    pub fn add_error(&mut self) {
        self.error_count += 1;
    }
    
    pub fn last_activity(&self) -> Instant {
        self.last_activity
    }
    
    pub fn errors(&self) -> u32 {
        self.error_count
    }
    
    pub fn recovery_attempts_increase(&mut self) {
        self.recovery_attempts += 1;
    }
    
    pub fn recovery_attempts(&self) -> u32 {
        self.recovery_attempts
    }
}
