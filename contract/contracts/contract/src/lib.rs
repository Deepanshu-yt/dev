#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Vec, Address, Map};

#[contracttype]
#[derive(Clone)]
pub struct Poll {
    pub id: u32,
    pub question: Symbol,
    pub options: Vec<Symbol>,
}

#[contracttype]
#[derive(Clone)]
pub struct VoteCount {
    pub counts: Map<Symbol, u32>,
}

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {

    // Create a new poll (permissionless)
    pub fn create_poll(env: Env, poll_id: u32, question: Symbol, options: Vec<Symbol>) {
        let poll = Poll {
            id: poll_id,
            question,
            options: options.clone(),
        };

        env.storage().instance().set(&poll_id, &poll);

        // Initialize vote counts
        let mut counts = Map::new(&env);
        for opt in options.iter() {
            counts.set(opt.clone(), 0);
        }

        let vote_count = VoteCount { counts };
        env.storage().instance().set(&(poll_id, Symbol::new(&env, "votes")), &vote_count);
    }

    // Vote on a poll (one vote per address)
    pub fn vote(env: Env, poll_id: u32, voter: Address, option: Symbol) {
        voter.require_auth();

        // Check if already voted
        let vote_key = (poll_id, voter.clone());
        if env.storage().instance().has(&vote_key) {
            panic!("Already voted!");
        }

        // Get vote counts
        let key = (poll_id, Symbol::new(&env, "votes"));
        let mut vote_count: VoteCount = env.storage().instance().get(&key).unwrap();

        // Increase count
        let current = vote_count.counts.get(option.clone()).unwrap_or(0);
        vote_count.counts.set(option, current + 1);

        // Save updated counts
        env.storage().instance().set(&key, &vote_count);

        // Mark voter as voted
        env.storage().instance().set(&vote_key, &true);
    }

    // Get poll details
    pub fn get_poll(env: Env, poll_id: u32) -> Poll {
        env.storage().instance().get(&poll_id).unwrap()
    }

    // Get results
    pub fn get_results(env: Env, poll_id: u32) -> Map<Symbol, u32> {
        let key = (poll_id, Symbol::new(&env, "votes"));
        let vote_count: VoteCount = env.storage().instance().get(&key).unwrap();
        vote_count.counts
    }
}
