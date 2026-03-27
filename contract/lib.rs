#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Map, String, Symbol, Vec,
};

// ─── Data Types ───────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug)]
pub struct WorkExperience {
    pub company: String,
    pub role: String,
    pub start_year: u32,
    pub end_year: u32,   // 0 = present
    pub description: String,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Education {
    pub institution: String,
    pub degree: String,
    pub field: String,
    pub graduation_year: u32,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Certification {
    pub name: String,
    pub issuer: String,
    pub issued_year: u32,
    pub credential_id: String,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Resume {
    pub owner: Address,
    pub name: String,
    pub title: String,
    pub bio: String,
    pub email: String,
    pub github: String,
    pub linkedin: String,
    pub skills: Vec<String>,
    pub experiences: Vec<WorkExperience>,
    pub education: Vec<Education>,
    pub certifications: Vec<Certification>,
    pub endorsed_by: Vec<Address>,   // peers who endorsed this resume
    pub created_at: u64,
    pub updated_at: u64,
}

// ─── Storage Keys ─────────────────────────────────────────────────────────────

const RESUME_KEY: Symbol = symbol_short!("RESUME");
const ENDORSE_KEY: Symbol = symbol_short!("ENDORSE");

// ─── Contract ─────────────────────────────────────────────────────────────────

#[contract]
pub struct OnChainResume;

#[contractimpl]
impl OnChainResume {
    // ── Create or overwrite a resume ─────────────────────────────────────────
    pub fn create_resume(
        env: Env,
        owner: Address,
        name: String,
        title: String,
        bio: String,
        email: String,
        github: String,
        linkedin: String,
        skills: Vec<String>,
    ) -> Resume {
        owner.require_auth();

        let now = env.ledger().timestamp();

        // Keep existing endorsements if resume already exists
        let endorsed_by: Vec<Address> = env
            .storage()
            .persistent()
            .get::<Address, Resume>(&owner)
            .map(|r| r.endorsed_by)
            .unwrap_or_else(|| Vec::new(&env));

        let resume = Resume {
            owner: owner.clone(),
            name,
            title,
            bio,
            email,
            github,
            linkedin,
            skills,
            experiences: Vec::new(&env),
            education: Vec::new(&env),
            certifications: Vec::new(&env),
            endorsed_by,
            created_at: now,
            updated_at: now,
        };

        env.storage()
            .persistent()
            .set::<Address, Resume>(&owner, &resume);

        env.events()
            .publish((RESUME_KEY, symbol_short!("created")), owner);

        resume
    }

    // ── Add a work-experience entry ──────────────────────────────────────────
    pub fn add_experience(
        env: Env,
        owner: Address,
        company: String,
        role: String,
        start_year: u32,
        end_year: u32,
        description: String,
    ) {
        owner.require_auth();

        let mut resume = Self::get_resume(env.clone(), owner.clone());

        resume.experiences.push_back(WorkExperience {
            company,
            role,
            start_year,
            end_year,
            description,
        });
        resume.updated_at = env.ledger().timestamp();

        env.storage()
            .persistent()
            .set::<Address, Resume>(&owner, &resume);
    }

    // ── Add an education entry ───────────────────────────────────────────────
    pub fn add_education(
        env: Env,
        owner: Address,
        institution: String,
        degree: String,
        field: String,
        graduation_year: u32,
    ) {
        owner.require_auth();

        let mut resume = Self::get_resume(env.clone(), owner.clone());

        resume.education.push_back(Education {
            institution,
            degree,
            field,
            graduation_year,
        });
        resume.updated_at = env.ledger().timestamp();

        env.storage()
            .persistent()
            .set::<Address, Resume>(&owner, &resume);
    }

    // ── Add a certification ──────────────────────────────────────────────────
    pub fn add_certification(
        env: Env,
        owner: Address,
        name: String,
        issuer: String,
        issued_year: u32,
        credential_id: String,
    ) {
        owner.require_auth();

        let mut resume = Self::get_resume(env.clone(), owner.clone());

        resume.certifications.push_back(Certification {
            name,
            issuer,
            issued_year,
            credential_id,
        });
        resume.updated_at = env.ledger().timestamp();

        env.storage()
            .persistent()
            .set::<Address, Resume>(&owner, &resume);
    }

    // ── Endorse someone's resume (peer endorsement) ──────────────────────────
    pub fn endorse(env: Env, endorser: Address, resume_owner: Address) {
        endorser.require_auth();

        // Can't endorse yourself
        if endorser == resume_owner {
            panic!("cannot endorse your own resume");
        }

        let mut resume = Self::get_resume(env.clone(), resume_owner.clone());

        // Prevent duplicate endorsements
        for existing in resume.endorsed_by.iter() {
            if existing == endorser {
                panic!("already endorsed");
            }
        }

        resume.endorsed_by.push_back(endorser.clone());
        resume.updated_at = env.ledger().timestamp();

        env.storage()
            .persistent()
            .set::<Address, Resume>(&resume_owner, &resume);

        env.events()
            .publish((ENDORSE_KEY, symbol_short!("endorsed")), (endorser, resume_owner));
    }

    // ── Read a resume ────────────────────────────────────────────────────────
    pub fn get_resume(env: Env, owner: Address) -> Resume {
        env.storage()
            .persistent()
            .get::<Address, Resume>(&owner)
            .expect("resume not found")
    }

    // ── Check if a resume exists ─────────────────────────────────────────────
    pub fn has_resume(env: Env, owner: Address) -> bool {
        env.storage()
            .persistent()
            .has::<Address, Resume>(&owner)
    }

    // ── Return endorsement count ─────────────────────────────────────────────
    pub fn endorsement_count(env: Env, owner: Address) -> u32 {
        let resume = Self::get_resume(env, owner);
        resume.endorsed_by.len()
    }

    // ── Delete / revoke a resume ─────────────────────────────────────────────
    pub fn delete_resume(env: Env, owner: Address) {
        owner.require_auth();
        env.storage()
            .persistent()
            .remove::<Address, Resume>(&owner);

        env.events()
            .publish((RESUME_KEY, symbol_short!("deleted")), owner);
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    fn setup() -> (Env, Address, OnChainResumeClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, OnChainResume);
        let client = OnChainResumeClient::new(&env, &contract_id);
        let owner = Address::generate(&env);
        (env, owner, client)
    }

    #[test]
    fn test_create_resume() {
        let (env, owner, client) = setup();

        let resume = client.create_resume(
            &owner,
            &String::from_str(&env, "Alice Dev"),
            &String::from_str(&env, "Senior Rust Engineer"),
            &String::from_str(&env, "Building the decentralized future."),
            &String::from_str(&env, "alice@example.com"),
            &String::from_str(&env, "github.com/alice"),
            &String::from_str(&env, "linkedin.com/in/alice"),
            &soroban_sdk::vec![
                &env,
                String::from_str(&env, "Rust"),
                String::from_str(&env, "Soroban"),
                String::from_str(&env, "WebAssembly"),
            ],
        );

        assert_eq!(resume.name, String::from_str(&env, "Alice Dev"));
        assert!(client.has_resume(&owner));
    }

    #[test]
    fn test_add_experience() {
        let (env, owner, client) = setup();
        client.create_resume(
            &owner,
            &String::from_str(&env, "Bob"),
            &String::from_str(&env, "Dev"),
            &String::from_str(&env, "Bio"),
            &String::from_str(&env, "bob@b.com"),
            &String::from_str(&env, ""),
            &String::from_str(&env, ""),
            &soroban_sdk::vec![&env],
        );

        client.add_experience(
            &owner,
            &String::from_str(&env, "Stellar Foundation"),
            &String::from_str(&env, "Smart Contract Engineer"),
            &2022,
            &0, // present
            &String::from_str(&env, "Building Soroban contracts."),
        );

        let resume = client.get_resume(&owner);
        assert_eq!(resume.experiences.len(), 1);
    }

    #[test]
    fn test_endorsement() {
        let (env, owner, client) = setup();
        let peer = Address::generate(&env);

        client.create_resume(
            &owner,
            &String::from_str(&env, "Charlie"),
            &String::from_str(&env, "Dev"),
            &String::from_str(&env, "Bio"),
            &String::from_str(&env, "c@c.com"),
            &String::from_str(&env, ""),
            &String::from_str(&env, ""),
            &soroban_sdk::vec![&env],
        );

        client.endorse(&peer, &owner);
        assert_eq!(client.endorsement_count(&owner), 1);
    }

    #[test]
    #[should_panic(expected = "cannot endorse your own resume")]
    fn test_self_endorse_fails() {
        let (env, owner, client) = setup();
        client.create_resume(
            &owner,
            &String::from_str(&env, "Dave"),
            &String::from_str(&env, "Dev"),
            &String::from_str(&env, ""),
            &String::from_str(&env, ""),
            &String::from_str(&env, ""),
            &String::from_str(&env, ""),
            &soroban_sdk::vec![&env],
        );
        client.endorse(&owner, &owner);
    }
}