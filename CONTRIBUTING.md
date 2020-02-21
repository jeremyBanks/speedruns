Feel free to submit issues or send PRs, but (at least for now) this is somewhat
meant as a personal project for experimenting with different tools, so I
reserve the right to make bad choices for arbtirary reasons.

By intentionally submitting code for inclusion in this repository you are
agreeing to release it under the terms of [the MIT License](LICENSE).

## Repository Automation

I use [GitHub Actions](.github/workflows/) and
[ZEIT](https://zeit.co/) to automate this repository.

### Automatic fixes

An Action will automatically apply fixes from tools including `prettier`,
`eslint`, and `rust fix` to any branches or pull requests in this repository.
Feel free to force-push over the these commits if you want. Please make liberal
use of ignore annotations if you disagree with the fixes: they're here to be
helpful, not perscriptive.

### Green first-parent chain on `master`

A GitHub Action will ensure that first-parent ancestors of every commit on
`master` has always passed CI. If `master` looks like, where `A`, `B`, and `C` are commits which already passed CI:

```
A🟢---B🟢---C🟢
```

and you push up three more commits at once, CI is only guaraunteed to test the
latest one, and we don't want the others in our first-parent ancestor chain.

```
A🟢---B🟢---C🟢---D❓---E❓---F🟢
```

The Action will add a merge commit like this to maintain the structure:

```
A🟢---B🟢---C🟢---------------------G🟢
               \---D❓---E❓---F🟢---/
```

If you push a broken commit to master, it'll sit there until you push a passing
commit. Don't do that.

### Deployment and packaging

There is currently only one instance of the Rust GraphQL backend, running our
latest release at `https://graphql-v0.speedrun.ca/`.

I use ZEIT to run instances of the frontend for almost every commit, branch,
and pull request. Their bot should comment on the commit or PR with a link
once it's been deployed. Most of these URLs will be randomly generated, but
we have a few standard ones:

https://speedruns.ca/ runs the latest release, matching the `deploy` branch.  
https://staging.speedruns.ca/ runs the latest version of `master`, which will
eventually be promoted to `deploy` unless the release fails.  
https://dev.speedruns.ca/ runs the `dev` branch, which is nothing special, just
for convenience if I want to put up a branch at a memorable URL.

An Action attempts to publish a `-dev` prerelease of this package for every
green commit on `master`, and then promotes it to `prod` if successful.

The backend is published as
[`speedruns` on crates.io](https://crates.io/crates/speedruns).

The frontend is published as
[`speedruns` on NPM](https://www.npmjs.com/package/speedruns) and as
[`@jeremybanks/speedruns` on NPM](https://www.npmjs.com/package/@jeremybanks/speedruns), and
[on the GitHub Package Registry](https://github.com/jeremyBanks/speedruns/packages/120812).
