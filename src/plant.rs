

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlantStage {
    Seed,
    Sprout,
    Seedling,
    YoungPlant,
    FullGrownPlant,
}

impl std::fmt::Display for PlantStage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PlantStage {
    pub fn from_u32(stage: u32) -> Self {
        match stage {
            0 => PlantStage::Seed,
            1 => PlantStage::Sprout,
            2 => PlantStage::Seedling,
            3 => PlantStage::YoungPlant,
            4 => PlantStage::FullGrownPlant,
            _ => PlantStage::FullGrownPlant,
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            PlantStage::Seed => 0,
            PlantStage::Sprout => 1,
            PlantStage::Seedling => 2,
            PlantStage::YoungPlant => 3,
            PlantStage::FullGrownPlant => 4,
        }
    }

    pub fn max_stage() -> u32 {
        4
    }



    pub fn icon(&self) -> &'static str {
        match self {
            PlantStage::Seed => "🌰",
            PlantStage::Sprout => "🍃",
            PlantStage::Seedling => "🌱",
            PlantStage::YoungPlant => "🌿",
            PlantStage::FullGrownPlant => "🪴",
        }
    }

}

#[derive(Debug, Clone)]
pub struct Plant {
    pub stage: PlantStage,
    pub growth_points: u32,
}

impl Plant {
    pub fn new() -> Self {
        Plant {
            stage: PlantStage::Seed,
            growth_points: 0,
        }
    }

    pub fn from_stage(stage: u32, points: u32) -> Self {
        Plant {
            stage: PlantStage::from_u32(stage),
            growth_points: points,
        }
    }

    pub fn add_growth(&mut self) {
        if self.growth_points < 10 {
            self.growth_points += 1;
            self.update_stage();
        }
    }

    pub fn update_stage(&mut self) {
        self.stage = match self.growth_points {
            0..=2 => PlantStage::Seed,
            3..=5 => PlantStage::Sprout,
            6..=8 => PlantStage::Seedling,
            9 => PlantStage::YoungPlant,
            10.. => PlantStage::FullGrownPlant,
        };
    }

    pub fn is_complete(&self) -> bool {
        self.growth_points >= 10
    }

    pub fn progress(&self) -> f64 {
        self.growth_points as f64 / 10.0
    }

    pub fn sessions_to_next_stage(&self) -> u32 {
        match self.growth_points {
            0..=2 => 3 - self.growth_points,
            3..=5 => 6 - self.growth_points,
            6..=8 => 9 - self.growth_points,
            9 => 1,
            _ => 0,
        }
    }
}