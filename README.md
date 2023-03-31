# GitLab report commenter

[GitLab][gitlab] is a great piece of software. Its CI is especially useful.

Their recent inclusion of [security CI/CD pipelines][sast] is also really interesting. However, their reporting functionnality is locked behind their "Ultimate" license, which is a tad bit expensive for my wallet.

This project aims to fix that by posting a user-friendly report as a comment on the merge request.

## Installation

Simply grab a binary from the release page & execute it on your CI pipeline, like so:

```yaml
stages:
  - check
sast:
  stage: check
  # fail early & often
  dependencies: []
  needs: []
  # install this binary
  before_script:
    - wget -O gitlab-report-commentator https://the-url.you.can/find-me
    - chmod +x gitlab-report-commentator
  variables:
    # you need to specify in the secrets. This token _must_ have the api permission.
    GITLAB_USER_TOKEN: $GITLAB_USER_TOKEN
  script:
    # Run your analyzer. (if you use gitlab sast, you will need to run a job with the sast run as a dependency/need and get its artifacts
    - cargo audit --json > audit.json
    # another nifty piece of software https://lib.rs/crates/gitlab-report
    - gitlab-report -p audit < audit.json > sast.json
    # Finally, convert the sast report to a markdown comment.
    # We will attempt to post it to the merge request's comments (aka "notes")
    - gitlab-report-commentator -t sast < sast.json
```

## TODO

- [ ] ability to fail the job given a max number of vulnerabilities of each type.
- [ ] implement GitLab's discussion API to post the review directly to the diff.

[gitlab]: https://about.gitlab.com/
[sast]: https://docs.gitlab.com/ee/user/application_security/sast/
