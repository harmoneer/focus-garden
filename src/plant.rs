

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlantStage {
    Seed,
    Sprout,
    Flower,
    MiniTree,
}

impl PlantStage {
    pub fn from_u32(stage: u32) -> Self {
        match stage {
            0 => PlantStage::Seed,
            1 => PlantStage::Sprout,
            2 => PlantStage::Flower,
            3 => PlantStage::MiniTree,
            _ => PlantStage::MiniTree,
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            PlantStage::Seed => 0,
            PlantStage::Sprout => 1,
            PlantStage::Flower => 2,
            PlantStage::MiniTree => 3,
        }
    }

    pub fn max_stage() -> u32 {
        3
    }

    pub fn icon(&self) -> &'static str {
        match self {
            PlantStage::Seed => "🌱",
            PlantStage::Sprout => "🌿",
            PlantStage::Flower => "🌸",
            PlantStage::MiniTree => "🌳",
        }
    }

    pub fn template_points(&self) -> Vec<(f64, f64)> {
        match self {
            PlantStage::Seed => vec![(0.0, 0.0)],
            PlantStage::Sprout => vec![(0.0, 0.0), (0.0, 1.0)],
            PlantStage::Flower => vec![(0.0, 0.0), (0.0, 1.0), (-1.0, 2.0), (1.0, 2.0)],
            PlantStage::MiniTree => vec![
                (0.0, 0.0), (0.0, 1.0), (0.0, 2.0), (-1.0, 3.0), (0.0, 3.0), (1.0, 3.0),
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Plant {
    pub stage: PlantStage,
    pub growth_points: Vec<(u16, u16)>,
}

impl Plant {
    pub fn new() -> Self {
        Plant {
            stage: PlantStage::Seed,
            growth_points: vec![],
        }
    }

    pub fn from_stage(stage: u32, points: Vec<(u16, u16)>) -> Self {
        Plant {
            stage: PlantStage::from_u32(stage),
            growth_points: points,
        }
    }

    pub fn add_growth(&mut self) {
        let template = self.stage.template_points();
        if self.growth_points.len() < template.len() {
            let (x, y) = template[self.growth_points.len()];
            self.growth_points.push((x as u16, y as u16));
        } else {
            // Advance to next stage
            let next_stage = self.stage.to_u32() + 1;
            if next_stage <= PlantStage::max_stage() {
                self.stage = PlantStage::from_u32(next_stage);
                self.growth_points = vec![];
            }
        }
    }

    pub fn is_complete(&self) -> bool {
        self.stage.to_u32() == PlantStage::max_stage() && self.growth_points.len() == self.stage.template_points().len()
    }

    pub fn progress(&self) -> f64 {
        let total_points = self.stage.template_points().len() as f64;
        self.growth_points.len() as f64 / total_points
    }

    pub fn sessions_to_next_stage(&self) -> usize {
        self.stage.template_points().len() - self.growth_points.len()
    }
}