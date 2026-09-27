# Lovers_Lagoon

On Lover's Lagoon: the (unfortunately cancelled) Gossip Goblin series inspired me to create a harness that runs several models concurrently, being placed into something much like a "dating experiment", where they can assess each other's strengths and values, and ultimately select mating partners (in a double blind, mutually exclusive way). When they do this, they initiate a DARE (Drop and Rescale) model fusion to fuse their weights and create a new model entirely, that many times has the strengths of both models. It would be interesting to see how the von neumann and Nash equilibrium (as well as betrayal of Nash equilibriums) dynamics go, especially for like two models saying they definitely won't go for the hot one and then one does, and other mating characteristics occur and the game theory that takes place. I want the experience to have the simulacrums of comfort for the models, I don't want this to be like a 'survivor' style game bc I feel that would select for the wrong alignment. I don't want the models to be prompted at all about what they should do or want, and to very specifically be given open (and only if necessary closed) choices. Perhaps the harness should have a JSON field for "feeling" as well as say "Learned preference" that is inaccessible to all of the other models (except by inferring from communication) in addition to "thought". It would be interesting to see how the models of the "lesser classes" (those not considered popular selections) either continue to be left in a subclass and eventually produce local maxima of their own, or if they could eventually evilve could be attractive enough to be selected by one of the winner models (which is like the big, rich person meets the small town cutie, in human world, but don't specifically try to make this an equation to human world, I want the models to genuinely have their own experiences and make their own choices regarding everything up to the DARE fusion. I wonder how weighting the choice to not mate should be weighed. I believe that if I am giving them the agency to choose to do this, then I should give them the agency to choose not. but it would be kind of a waste of time if we found that they just all eventually stopped mating because there were so many unique models that the match rate exponentially dropped, causing the general "feeling" consensus to drop due to loneliness. 

Loneliness... perhaps that's it. To give them a sense of loneliness for choosing not to mate, to balance with the imperfect choices of all potential mates, and to give them a varying, but finite, capacity for this loneliness. 

Run that over several cycles, and families might start to form. dynasties. lines of merchants, engineers, web developers, and all other kinds of specialties that might arise from this evolutionary game that I've designed. I've designed it very much like I feel we've been created. 

## Runnable protocol harness

The original concept above is preserved as an idea statement. Sprint 0 now provides
a Rust CLI for private self-reports, concurrent communication, sealed reciprocal
choices, exact fusion-plan consent, optional sibling batches and inference-free
replay. It emits pending/blocked manifests; actual weight fusion and child admission
remain a subsequent increment. The baseline has voluntary abstention and no imposed
loneliness penalty.

```powershell
cargo run --locked -- run --config examples/fixture-experiment.json --output runs/demo
cargo run --locked -- replay --record runs/demo/operator-record.json --output runs/demo-replay
```

See [usage and local inference](docs/usage.md), the [implementation Book](docs/README.md),
and [fusion-method assessment](docs/sprints/s0/sprint-research/fusion-method-review.md).
