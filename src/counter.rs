#[derive(Clone, Copy)]
struct Count<Type: Eq + Copy> {
    id: Option<Type>,
    amount: u64
}

#[derive(Clone, Copy)]
pub struct ConsecutiveCounter<Type: Eq + Copy> {
    current: Count<Type>,
    completed: Option<Count<Type>>
}

impl<Type: Eq + Copy> ConsecutiveCounter<Type> {
    pub fn new() -> ConsecutiveCounter<Type> {
        ConsecutiveCounter {
            current: Count {
                id: None,
                amount: 0
            },
            completed: None
        }
    }

    pub fn add_for(&self, id: Type, amount: u64) -> ConsecutiveCounter<Type> {
        if Some(id) == self.current.id {
            ConsecutiveCounter {
                current: Count {
                    id: Some(id),
                    amount: self.current.amount + amount
                },
                completed: None
            }
        } else {
            ConsecutiveCounter {
                current: Count {
                    id: Some(id),
                    amount: amount
                },
                completed: if self.current.id == None {
                    None
                } else {
                    Some(self.current)
                }
            }
        }
    }

    pub fn completed_count(&self) -> Option<(Type, u64)> {
        self.completed.map(|count| (count.id.unwrap(), count.amount))
    }
}
