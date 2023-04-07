# GitLab report commenter

[GitLab][gitlab] is a great piece of software. Its CI is especially useful.

Their recent inclusion of [security CI/CD pipelines][sast] is also fascinating. However, their reporting functionality is locked behind their "Ultimate" licence, which is a tad bit expensive for my wallet.

This project aims to fix that by posting a user-friendly report as a comment on the merge request.

```
Usage: gitlab-report-commentator [OPTIONS] --type <REPORT_TYPE>

Options:
  -t, --type <REPORT_TYPE>  [possible values: sast]
  -m, --mode <MODE>         [default: mr-comment] [possible values: mr-comment, diff-comment]
  -d, --dry-run             Don't try to actually post the message, just parse the report.
  -h, --help                Print help
  -V, --version             Print version
```

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

If you're using gitlab SAST, it's even simpler:
```yaml
include:
  - template: Security/SAST.gitlab-ci.yml
sast:
  # fail early & often
  dependencies: []
  needs: []
  variables:
    GL_REPORT_COMMENTATOR_VERSION: v0.1.0+alpha.3
    GITLAB_USER_TOKEN: my-token-is-a-long-string-supposed-to-be-secret-but-for-the-sake-of-demonstration-i-put-it-in-plain-text-in-my-commited-gitlabci.yaml-file
  stage: test
  after_script:
    - test -f gl-sast-report.json &&
      wget -O gitlab-report-commentator https://github.com/ololduck/gitlab-report-commentor/releases/download/GL_REPORT_COMMENTATOR_VERSION/gitlab-report-commentator.x86_64-musl &&
      chmod +x gitlab-report-commentator &&
      ./gitlab-report-commentator -t sast <gl-sast-report.json
```

WARNING: using `after_script` will prevent you from failing the job if `gitlab-report-commentator`'s execution fails!

## TODO

- [ ] ability to fail the job given a max number of vulnerabilities of each type.
- [x] implement GitLab's discussion API to post the review directly to the diff.
- [ ] Cleanup the code & use more efficient types, such as a 28 `char` array for SHA-1 hashes instead of a `String`. Some behaviour could also be reworked for the sake of simplicity.

[gitlab]: https://about.gitlab.com/
[sast]: https://docs.gitlab.com/ee/user/application_security/sast/
