//! 链上治理 - 提案、投票、链上升级
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub proposer: String,
    pub votes_for: u64,
    pub votes_against: u64,
    pub end_time: u64,
    pub executed: bool,
}

pub struct GovernanceCore {
    proposals: HashMap<u64, Proposal>,
    next_id: u64,
    voting_period: u64,
    min_stake: u64,
}

impl GovernanceCore {
    pub fn new(voting_period: u64, min_stake: u64) -> Self {
        Self {
            proposals: HashMap::new(),
            next_id: 1,
            voting_period,
            min_stake,
        }
    }

    pub fn create_proposal(&mut self, title: String, description: String, proposer: String, current_time: u64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let proposal = Proposal {
            id,
            title,
            description,
            proposer,
            votes_for: 0,
            votes_against: 0,
            end_time: current_time + self.voting_period,
            executed: false,
        };
        self.proposals.insert(id, proposal);
        id
    }

    pub fn vote(&mut self, proposal_id: u64, voter: &str, stake: u64, approve: bool) -> bool {
        if stake < self.min_stake {
            return false;
        }
        let proposal = self.proposals.get_mut(&proposal_id)?;
        if approve {
            proposal.votes_for += stake;
        } else {
            proposal.votes_against += stake;
        }
        true
    }

    pub fn execute_proposal(&mut self, proposal_id: u64, current_time: u64) -> bool {
        let proposal = self.proposals.get_mut(&proposal_id)?;
        if current_time < proposal.end_time || proposal.executed {
            return false;
        }
        proposal.executed = proposal.votes_for > proposal.votes_against;
        proposal.executed
    }
}
