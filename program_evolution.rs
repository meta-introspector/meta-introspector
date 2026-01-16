// Program evolution: Genetic algorithm on complexity lattice
// Programs as DNA evolving in crossbeam parallel world

use crossbeam::channel::{bounded, Sender, Receiver};
use crossbeam::thread;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ProgramDNA {
    pub id: u64,
    pub code: Vec<u8>,
    pub vector: Vec<f64>,
    pub complexity: usize,
    pub godel_number: u64,
    pub fitness: f64,
    pub generation: usize,
    pub parents: Vec<u64>,
}

impl ProgramDNA {
    pub fn random(size: usize) -> Self {
        use crate::rand_shim::random_u64;
        let code: Vec<u8> = (0..size).map(|_| (random_u64() & 0xFF) as u8).collect();
        
        Self {
            id: random_u64(),
            code,
            vector: Vec::new(),
            complexity: 0,
            godel_number: 0,
            fitness: 0.0,
            generation: 0,
            parents: Vec::new(),
        }
    }
    
    pub fn compute_properties(&mut self) {
        self.complexity = self.code.len();
        self.godel_number = self.compute_godel();
        self.vector = self.compute_vector();
    }
    
    fn compute_godel(&self) -> u64 {
        let mut godel = 1u64;
        let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
        
        for (i, &byte) in self.code.iter().take(15).enumerate() {
            godel = godel.wrapping_mul((primes[i] as u64).wrapping_pow(byte as u32));
        }
        
        godel
    }
    
    fn compute_vector(&self) -> Vec<f64> {
        vec![
            self.complexity as f64,
            (self.godel_number as f64 + 1.0).ln(),
            (self.godel_number % 71) as f64,
        ]
    }
}

pub struct EvolutionWorld {
    pub programs: Vec<ProgramDNA>,
    pub generation: usize,
    pub mutation_rate: f64,
    pub crossover_rate: f64,
}

impl EvolutionWorld {
    pub fn new(population_size: usize) -> Self {
        let mut programs = Vec::new();
        
        for _ in 0..population_size {
            let mut program = ProgramDNA::random(64);
            program.compute_properties();
            programs.push(program);
        }
        
        Self {
            programs,
            generation: 0,
            mutation_rate: 0.01,
            crossover_rate: 0.7,
        }
    }
    
    pub fn run(&mut self, generations: usize, workers: usize) {
        for gen in 0..generations {
            self.generation = gen;
            
            println!("Generation {}", gen);
            
            // Evaluate fitness in parallel
            self.evaluate_parallel(workers);
            
            // Selection
            let selected = self.select();
            
            // Crossover and mutation
            self.programs = self.reproduce(selected);
            
            // Report
            self.report();
        }
    }
    
    fn evaluate_parallel(&mut self, workers: usize) {
        let (tx, rx) = bounded(self.programs.len());
        
        thread::scope(|s| {
            // Spawn workers
            for _ in 0..workers {
                let programs = self.programs.clone();
                let tx = tx.clone();
                
                s.spawn(move |_| {
                    for program in programs {
                        let fitness = compute_fitness(&program);
                        tx.send((program.id, fitness)).unwrap();
                    }
                });
            }
            
            drop(tx);
            
            // Collect results
            for (id, fitness) in rx {
                if let Some(program) = self.programs.iter_mut().find(|p| p.id == id) {
                    program.fitness = fitness;
                }
            }
        }).unwrap();
    }
    
    fn select(&self) -> Vec<ProgramDNA> {
        let mut selected = Vec::new();
        
        // Tournament selection
        use crate::rand_shim::random_usize;
        for _ in 0..self.programs.len() {
            let mut tournament = Vec::new();
            for _ in 0..3 {
                let idx = random_usize() % self.programs.len();
                tournament.push(&self.programs[idx]);
            }
            
            let winner = tournament.iter()
                .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
                .unwrap();
            
            selected.push((*winner).clone());
        }
        
        selected
    }
    
    fn reproduce(&mut self, selected: Vec<ProgramDNA>) -> Vec<ProgramDNA> {
        let mut next_gen = Vec::new();
        
        for i in (0..selected.len()).step_by(2) {
            let parent1 = &selected[i];
            let parent2 = &selected[(i + 1) % selected.len()];
            
            // Crossover
            use crate::rand_shim::{random_usize, random_f64, random_u64};
            let mut child = if random_f64() < self.crossover_rate {
                self.crossover(parent1, parent2)
            } else {
                parent1.clone()
            };
            
            // Mutation
            self.mutate(&mut child);
            
            child.compute_properties();
            next_gen.push(child);
        }
        
        next_gen
    }
    
    fn crossover(&self, parent1: &ProgramDNA, parent2: &ProgramDNA) -> ProgramDNA {
        use crate::rand_shim::{random_usize, random_u64};
        let point = random_usize() % parent1.code.len().min(parent2.code.len());
        
        let mut code = Vec::new();
        code.extend_from_slice(&parent1.code[..point]);
        code.extend_from_slice(&parent2.code[point..]);
        
        ProgramDNA {
            id: random_u64(),
            code,
            vector: Vec::new(),
            complexity: 0,
            godel_number: 0,
            fitness: 0.0,
            generation: self.generation + 1,
            parents: vec![parent1.id, parent2.id],
        }
    }
    
    fn mutate(&self, program: &mut ProgramDNA) {
        use crate::rand_shim::{random_f64, random_u64};
        for byte in &mut program.code {
            if random_f64() < self.mutation_rate {
                *byte = (random_u64() & 0xFF) as u8;
            }
        }
    }
    
    fn report(&self) {
        let avg_fitness: f64 = self.programs.iter().map(|p| p.fitness).sum::<f64>() 
            / self.programs.len() as f64;
        
        let best = self.programs.iter()
            .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
            .unwrap();
        
        println!("  Avg fitness: {:.2}", avg_fitness);
        println!("  Best fitness: {:.2}", best.fitness);
        println!("  Best complexity: {}", best.complexity);
        println!("  Best Gödel: {}", best.godel_number);
        println!();
    }
}

fn compute_fitness(program: &ProgramDNA) -> f64 {
    let mut fitness = 0.0;
    
    // Simplicity (smaller is better)
    fitness += 1.0 / (program.complexity as f64 + 1.0) * 50.0;
    
    // Diversity (unique Gödel number)
    fitness += (program.godel_number as f64).ln() * 0.1;
    
    // Orbit preference (prime orbits are better)
    let orbit = program.godel_number % 71;
    if is_prime(orbit) {
        fitness += 25.0;
    }
    
    fitness
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    
    let sqrt = (n as f64).sqrt() as u64;
    for i in (3..=sqrt).step_by(2) {
        if n % i == 0 { return false; }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_program_dna() {
        let mut program = ProgramDNA::random(32);
        program.compute_properties();
        
        assert_eq!(program.complexity, 32);
        assert!(program.godel_number > 0);
        assert!(program.vector.len() > 0);
    }
    
    #[test]
    fn test_evolution() {
        let mut world = EvolutionWorld::new(10);
        world.run(5, 2);
        
        assert_eq!(world.generation, 4);
        assert!(world.programs.len() > 0);
    }
}
