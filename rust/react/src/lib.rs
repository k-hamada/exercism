use std::cell::Cell;

/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputCellID(usize);
/// `ComputeCellID` is a unique identifier for a compute cell.
/// Values of type `InputCellID` and `ComputeCellID` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellID = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellID = r.create_compute(&[react::CellID::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComputeCellID(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallbackID(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct InputCell<T> {
    value: Cell<T>,
    computes: Vec<ComputeCellID>,
}

struct ComputeCell<'a, T> {
    value: Cell<T>,
    compute_func: Box<dyn Fn(&[T]) -> T + 'a>,
    dependencies: Vec<CellID>,
    computes: Vec<ComputeCellID>,
    callbacks: Vec<Option<CallbackCell<'a, T>>>,
}

struct CallbackCell<'a, T> {
    value: Cell<T>,
    callback: Box<dyn FnMut(T) + 'a>,
}

pub struct Reactor<'a, T> {
    inputs: Vec<InputCell<T>>,
    computes: Vec<ComputeCell<'a, T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T> Reactor<'a, T>
where
    T: Copy + PartialEq + 'a,
{
    pub fn new() -> Self {
        Reactor {
            inputs: Default::default(),
            computes: Default::default(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        let input_cell_id = InputCellID(self.inputs.len());
        let input_cell = InputCell {
            value: Cell::new(initial),
            computes: Default::default(),
        };

        self.inputs.push(input_cell);

        input_cell_id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
        &mut self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID> {
        for dependency in dependencies {
            let exist = match dependency {
                CellID::Input(InputCellID(index)) => self.inputs.len() > *index,
                CellID::Compute(ComputeCellID(index)) => self.computes.len() > *index,
            };
            if !exist {
                return Err(*dependency);
            }
        }

        let value = self.compute_cell_value(dependencies, &compute_func);

        let compute_cell_id = ComputeCellID(self.computes.len());
        let compute_cell = ComputeCell {
            value: Cell::new(value),
            compute_func: Box::new(compute_func),
            dependencies: dependencies.to_vec(),
            computes: Default::default(),
            callbacks: Default::default(),
        };

        self.computes.push(compute_cell);

        for dependency in dependencies {
            match dependency {
                CellID::Input(InputCellID(index)) => {
                    if let Some(input_cell) = self.inputs.get_mut(*index) {
                        input_cell.computes.push(compute_cell_id);
                    }
                }
                CellID::Compute(ComputeCellID(index)) => {
                    if let Some(compute_cell) = self.computes.get_mut(*index) {
                        compute_cell.computes.push(compute_cell_id);
                    }
                }
            }
        }

        Ok(compute_cell_id)
    }

    fn compute_cell_value<F: Fn(&[T]) -> T + 'a>(
        &self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> T {
        let args = dependencies
            .iter()
            .flat_map(|dependency| self.value(*dependency))
            .collect::<Vec<T>>();
        compute_func(&args)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        match &id {
            CellID::Input(InputCellID(index)) => {
                self.inputs.get(*index).map(|cell| cell.value.get())
            }
            CellID::Compute(ComputeCellID(index)) => {
                self.computes.get(*index).map(|cell| cell.value.get())
            }
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        let input_cell = &self.inputs.get(id.0);
        if input_cell.is_none() {
            return false;
        }
        let input_cell = input_cell.unwrap();

        input_cell.value.set(new_value);

        for compute_cell_id in &input_cell.computes {
            self.call_compute_cell(&compute_cell_id);
        }

        for compute_cell_id in &input_cell.computes.clone() {
            self.call_callback_cell(&compute_cell_id);
        }

        true
    }

    fn call_compute_cell(&self, id: &ComputeCellID) {
        let compute_cell = &self.computes.get(id.0);
        if compute_cell.is_none() {
            return;
        }
        let compute_cell = compute_cell.unwrap();

        let value = self.compute_cell_value(&compute_cell.dependencies, &compute_cell.compute_func);

        compute_cell.value.set(value);

        for compute_cell_id in &compute_cell.computes {
            self.call_compute_cell(&compute_cell_id);
        }
    }

    fn call_callback_cell(&mut self, id: &ComputeCellID) {
        let compute_cell = &mut self.computes[id.0];
        let compute_cell_value = compute_cell.value.get();

        for callback_cell in &mut compute_cell.callbacks {
            if let Some(callback_cell) = callback_cell {
                if compute_cell_value == callback_cell.value.get() {
                    continue;
                }

                let callback = &mut callback_cell.callback;
                callback(compute_cell_value);

                callback_cell.value.set(compute_cell_value);
            }
        }

        for compute_cell_id in &compute_cell.computes.clone() {
            self.call_callback_cell(&compute_cell_id);
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        compute_id: ComputeCellID,
        callback: F,
    ) -> Option<CallbackID> {
        if let Some(compute_cell) = self.computes.get_mut(compute_id.0) {
            let callback_cell_id = CallbackID(compute_cell.callbacks.len());
            let callback_cell = CallbackCell {
                callback: Box::new(callback),
                value: compute_cell.value.clone(),
            };

            compute_cell.callbacks.push(Some(callback_cell));

            Some(callback_cell_id)
        } else {
            None
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        compute_id: ComputeCellID,
        callback_id: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        if let Some(compute_cell) = self.computes.get_mut(compute_id.0) {
            if std::mem::replace(&mut compute_cell.callbacks[callback_id.0], None).is_some() {
                Ok(())
            } else {
                Err(RemoveCallbackError::NonexistentCallback)
            }
        } else {
            Err(RemoveCallbackError::NonexistentCell)
        }
    }
}
