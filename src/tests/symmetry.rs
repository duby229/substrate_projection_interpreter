use sptl_spi::{agents::Agent, substrate::Pattern, symbol::Symbol, symmetry};

#[test]
fn test_attractor_detection() {
    let mut agent = Agent::new("a", 16, 0.1);
    let tau = 0;
    let s = agent.express_symbol("foo", Pattern::new("101"), tau);
    for i in 0..5 {
        agent.interpret_symbol(&s, tau + i);
    }
    // Attractor state should be true (interpretant descriptions all identical)
    assert!(agent.is_attractor_state(3));

    // Now mutate and interpret, breaking symmetry
    let s2 = agent.mutate_symbol(&s);
    agent.interpret_symbol(&s2, tau + 5);
    // Attractor state should now be false
    assert!(!agent.is_attractor_state(3));
}