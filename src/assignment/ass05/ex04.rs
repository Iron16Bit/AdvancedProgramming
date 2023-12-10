use std::path::Iter;

#[derive(Debug)]
pub struct Tasks {
    tasks: Vec<Task>,
}

#[derive(Clone, Debug)]
pub struct Task {
    name: String,
    priority: i32,
    done: bool,
}

impl Task {
    pub(crate) fn new(name: String, priority: i32, done: bool) -> Self {
        return Task {
            name,
            priority,
            done,
        };
    }
}

impl Tasks {
    pub(crate) fn new() -> Self {
        return Tasks { tasks: Vec::new() };
    }

    pub fn push(&mut self, t: Task) {
        let mut index = 0;
        for task in self.tasks.iter() {
            if task.priority > t.priority {
                index += 1;
            }
        }

        self.tasks.insert(index, t);
    }
}

impl Iterator for Tasks {
    type Item = Task;
    fn next(&mut self) -> Option<Self::Item> {
        return self
            .tasks
            .iter() //Iter through the tasks
            .position(|task| !task.done) //Puts the Task we're iterating on in task and checks if the condition is verified. If so, returns its index
            .map(|task| self.tasks.remove(task)); //Takes that index and removes that value
    }
}
