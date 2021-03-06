pub mod basic_data;
pub mod acct_info;
pub mod card;
pub mod accessory;
pub mod schedule;
pub mod song;
pub mod skill;
mod acc_handle;
mod live_show;

#[derive(Debug, Clone, PartialEq)]
pub struct PlayGlob {
    pub song: song::Song,
    pub album: Vec<card::Card>,
    pub inventory: Vec<accessory::Acc>,
}

impl PlayGlob {
    pub fn est_voltage(&self, sched: &schedule::Schedule) -> f64 {
        live_show::run(&self.song, &self.album, &self.inventory, &sched)
    }
}

const R: f64 = 1.0 / 1_00_00.0;
#[inline]
pub fn pct<T: Into<f64>>(n: T) -> f64 {
    n.into() * R
}