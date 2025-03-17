
use super::lib::*;

impl Calculation {
    /// Initializes a new `Calculation` instance with all fields set to their
    /// default values.
    pub fn new() -> Self {
        Self {
            display: 0.0,
            pre_num: 0.0,
            is_selected: false,
            select_mode: 0.0,
        }
    }


    /// Handles an input from the user.
    ///
    /// If `num` is 0-9, it is added to the current display.
    /// If `num` is 10, the display is cleared.
    /// If `num` is 11, the current calculation is evaluated.
    /// If `num` is 12, 13, 14, or 15, the respective operation is selected.
    pub fn input(&self, num: f64) -> Self{
        let mut  new_self = self.clone();
        if num >= 0.0 && num <= 9.0 {
            new_self.display = self.display * 10.0 + num;
        } else {
            new_self = new_self.calculation_selectong(num)
        }
        new_self
    }

    /// Handles selecting an operation.
    ///
    /// If `select` is 10, the current calculation is cleared.
    /// If `select` is 11, the current calculation is evaluated.
    /// If `select` is 12, 13, 14, or 15, the respective operation is selected.
    fn calculation_selectong(&self, select: f64) -> Self {
        let mut new_self = self.clone();
        match select {
            10.0 => {// clear
                new_self = Self::new();
            },
            11.0 => {// equal
                new_self = new_self.calculate();
            },
            12.0 => {// add
                new_self.select_mode = 0.0;
                new_self = new_self.change_select();
            },
            13.0 => {// subtract
                new_self.select_mode = 1.0;
                new_self = new_self.change_select();
            },
            14.0 => {// multiply
                new_self.select_mode = 2.0;
                new_self = new_self.change_select();
            },
            15.0 => {// divide
                new_self.select_mode = 3.0;
                new_self = new_self.change_select();
            },
            _ => {}
        }
        new_self
    }

    /// Changes the `Calculation` instance to be in the selected state.
    ///
    /// When the instance is not in the selected state, this method sets the
    /// `is_selected` field to `true`, sets the `pre_num` field to the current
    /// `display`, and resets the `display` to 0.
    ///
    /// When the instance is in the selected state, this method does nothing.
    fn change_select(&self) -> Self {
        let mut new_self = self.clone();
        if !self.is_selected {
            new_self.is_selected = true;
            new_self.pre_num = self.display;
            new_self.display = 0.0;
        }
        new_self
    }

    /// Performs the selected arithmetic operation on the stored `pre_num` and
    /// the current `display` value, updating the `display` with the result.
    ///
    /// The operation performed is determined by `select_mode`:
    /// - `0.0`: Addition
    /// - `1.0`: Subtraction
    /// - `2.0`: Multiplication
    /// - `3.0`: Division
    ///
    /// After the calculation, the `pre_num` is updated to the new `display`
    /// value, and the `is_selected` flag is reset to `false`.

    fn calculate(&self) -> Self {
        let mut new_self = self.clone();
        match self.select_mode {
            0.0 => {
                new_self.display = self.pre_num + self.display;
            },
            1.0 => {
                new_self.display = self.pre_num - self.display;
            },
            2.0 => {
                new_self.display = self.pre_num * self.display;
            },
            3.0 => {
                new_self.display = self.pre_num / self.display;
            },
            _ => {}
        }
        new_self.pre_num = new_self.display;
        new_self.is_selected = false;
        new_self
    }
}
