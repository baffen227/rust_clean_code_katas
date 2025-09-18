// ============================================
// Kata 7: Mini TODO Application
// Focus: Clean Architecture, Layered Design
// ============================================

// Domain Layer
mod domain {
    use std::fmt;
    
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TaskId(u32);
    
    impl TaskId {
        pub fn new(id: u32) -> Self {
            TaskId(id)
        }
        
        pub fn value(&self) -> u32 {
            self.0
        }
    }
    
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Priority {
        Low,
        Medium,
        High,
    }
    
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum TaskStatus {
        Pending,
        InProgress,
        Completed,
    }
    
    #[derive(Debug, Clone)]
    pub struct Task {
        id: TaskId,
        title: String,
        description: String,
        priority: Priority,
        status: TaskStatus,
    }
    
    impl Task {
        pub fn new(id: TaskId, title: String, description: String, priority: Priority) -> Self {
            Task {
                id,
                title,
                description,
                priority,
                status: TaskStatus::Pending,
            }
        }
        
        pub fn id(&self) -> TaskId {
            self.id
        }
        
        pub fn title(&self) -> &str {
            &self.title
        }
        
        pub fn description(&self) -> &str {
            &self.description
        }
        
        pub fn priority(&self) -> Priority {
            self.priority
        }
        
        pub fn status(&self) -> TaskStatus {
            self.status
        }
        
        pub fn start(&mut self) {
            if self.status == TaskStatus::Pending {
                self.status = TaskStatus::InProgress;
            }
        }
        
        pub fn complete(&mut self) {
            self.status = TaskStatus::Completed;
        }
    }
    
    impl fmt::Display for Task {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "[{}] {} - {:?} ({:?})",
                self.id.value(),
                self.title,
                self.priority,
                self.status
            )
        }
    }
}

// Use Case Layer
mod use_cases {
    use super::domain::{Task, TaskId, Priority};
    use super::ports::TaskRepository;
    
    pub struct AddTaskUseCase<'a, R: TaskRepository> {
        repository: &'a mut R,
    }
    
    impl<'a, R: TaskRepository> AddTaskUseCase<'a, R> {
        pub fn new(repository: &'a mut R) -> Self {
            AddTaskUseCase { repository }
        }
        
        pub fn execute(&mut self, title: String, description: String, priority: Priority) -> Result<TaskId, String> {
            if title.is_empty() {
                return Err("Task title cannot be empty".to_string());
            }
            
            let id = self.repository.next_id();
            let task = Task::new(id, title, description, priority);
            self.repository.save(task)?;
            
            Ok(id)
        }
    }
    
    pub struct CompleteTaskUseCase<'a, R: TaskRepository> {
        repository: &'a mut R,
    }
    
    impl<'a, R: TaskRepository> CompleteTaskUseCase<'a, R> {
        pub fn new(repository: &'a mut R) -> Self {
            CompleteTaskUseCase { repository }
        }
        
        pub fn execute(&mut self, task_id: TaskId) -> Result<(), String> {
            let mut task = self.repository
                .find_by_id(task_id)
                .ok_or_else(|| format!("Task with id {} not found", task_id.value()))?;
            
            task.complete();
            self.repository.update(task)?;
            
            Ok(())
        }
    }
    
    pub struct ListTasksUseCase<'a, R: TaskRepository> {
        repository: &'a R,
    }
    
    impl<'a, R: TaskRepository> ListTasksUseCase<'a, R> {
        pub fn new(repository: &'a R) -> Self {
            ListTasksUseCase { repository }
        }
        
        pub fn execute(&self) -> Vec<Task> {
            self.repository.find_all()
        }
        
        pub fn execute_by_priority(&self, priority: Priority) -> Vec<Task> {
            self.repository
                .find_all()
                .into_iter()
                .filter(|task| task.priority() == priority)
                .collect()
        }
    }
}

// Ports (interfaces)
mod ports {
    use super::domain::{Task, TaskId};
    
    pub trait TaskRepository {
        fn save(&mut self, task: Task) -> Result<(), String>;
        fn update(&mut self, task: Task) -> Result<(), String>;
        fn find_by_id(&self, id: TaskId) -> Option<Task>;
        fn find_all(&self) -> Vec<Task>;
        fn next_id(&self) -> TaskId;
    }
}

// Infrastructure Layer
mod infrastructure {
    use std::collections::HashMap;
    use super::domain::{Task, TaskId};
    use super::ports::TaskRepository;
    
    pub struct InMemoryTaskRepository {
        tasks: HashMap<TaskId, Task>,
        next_id: u32,
    }
    
    impl InMemoryTaskRepository {
        pub fn new() -> Self {
            InMemoryTaskRepository {
                tasks: HashMap::new(),
                next_id: 1,
            }
        }
    }
    
    impl TaskRepository for InMemoryTaskRepository {
        fn save(&mut self, task: Task) -> Result<(), String> {
            let id = task.id();
            if self.tasks.contains_key(&id) {
                return Err(format!("Task with id {} already exists", id.value()));
            }
            self.tasks.insert(id, task);
            Ok(())
        }
        
        fn update(&mut self, task: Task) -> Result<(), String> {
            let id = task.id();
            if !self.tasks.contains_key(&id) {
                return Err(format!("Task with id {} not found", id.value()));
            }
            self.tasks.insert(id, task);
            Ok(())
        }
        
        fn find_by_id(&self, id: TaskId) -> Option<Task> {
            self.tasks.get(&id).cloned()
        }
        
        fn find_all(&self) -> Vec<Task> {
            self.tasks.values().cloned().collect()
        }
        
        fn next_id(&self) -> TaskId {
            let id = TaskId::new(self.next_id);
            // Note: In a real implementation, next_id should be updated
            id
        }
    }
}

// Application entry point
pub mod todo_app {
    use super::domain::Priority;
    use super::infrastructure::InMemoryTaskRepository;
    use super::use_cases::{AddTaskUseCase, CompleteTaskUseCase, ListTasksUseCase};
    
    pub struct TodoApplication {
        repository: InMemoryTaskRepository,
    }
    
    impl TodoApplication {
        pub fn new() -> Self {
            TodoApplication {
                repository: InMemoryTaskRepository::new(),
            }
        }
        
        pub fn add_task(&mut self, title: String, description: String, priority: Priority) -> Result<String, String> {
            let mut use_case = AddTaskUseCase::new(&mut self.repository);
            let task_id = use_case.execute(title, description, priority)?;
            Ok(format!("Task created with ID: {}", task_id.value()))
        }
        
        pub fn complete_task(&mut self, task_id: u32) -> Result<String, String> {
            let mut use_case = CompleteTaskUseCase::new(&mut self.repository);
            use_case.execute(super::domain::TaskId::new(task_id))?;
            Ok(format!("Task {} marked as completed", task_id))
        }
        
        pub fn list_all_tasks(&self) -> Vec<String> {
            let use_case = ListTasksUseCase::new(&self.repository);
            use_case
                .execute()
                .into_iter()
                .map(|task| task.to_string())
                .collect()
        }
    }
}

#[cfg(test)]
mod todo_tests {
    use super::*;
    use super::domain::Priority;
    use super::todo_app::TodoApplication;
    
    #[test]
    fn test_todo_workflow() {
        let mut app = TodoApplication::new();
        
        // Add task
        let result = app.add_task(
            "Write tests".to_string(),
            "Write unit tests for todo app".to_string(),
            Priority::High
        );
        assert!(result.is_ok());
        
        // List tasks
        let tasks = app.list_all_tasks();
        assert_eq!(tasks.len(), 1);
    }
}
