use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct CreatorMemecoinCounter {
    pub count: u32,
}

impl CreatorMemecoinCounter {

    pub fn increment(
        &mut self,
    ) {
        self.count += 1;
    }
}
