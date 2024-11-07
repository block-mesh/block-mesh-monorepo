# Rewards

All bounty submissions are rated by *BlockMesh Network* using a purposefully simple scale.

Each vulnerability is unique, but the following is a rough guideline we use internally for rating and rewarding
submissions.

Please report the issue in a secure and private manner via `support@blockmesh.xyz`.
With a title of `Security vulnerability`.

## Critical - \$150+

Critical severity issues present a direct and immediate risk to a broad array of our users
or to a *BlockMesh Network* product itself.
They often affect relatively low-level/foundational components in one of our application stacks or infrastructure.

### For example:

* Arbitrary code/command execution on a server in our production network
* Arbitrary SQL queries on a production database
* Bypassing the login process, either password or 2FA
* Access to sensitive production user data or access to internal production systems
* Accessing another user’s data in the BlockMesh Network Actions service
* The upper bound for critical vulnerabilities, $30,000, is only a guideline, and BlockMesh Network may reward higher
  amounts for exceptional reports.

## High - \$100 - \$150

High severity issues allow an attacker to read or modify highly sensitive data that they are
not authorized to access.
They are generally more narrow in scope than critical issues,
though they may still grant an attacker extensive access.

### For example:

* Injecting attacker controlled content into BlockMesh Network.com (XSS) that bypasses CSP
* Bypassing authorization logic to grant a repository or package collaborator more access than intended
* Discovering sensitive user or BlockMesh Network data in a publicly exposed resource, such as an S3 bucket
* Overwriting a customer repository or package that should be inaccessible
* Gaining access to a non-critical resource that only employees should be able to reach
* Using the *BlockMesh Network* Actions repo-scoped GitHub token to access high-risk private content outside of that
  repository
* Sending authentication credentials from a client app to an unintended server
* Code execution in a client app that requires no user interaction, such as arbitrary code execution upon repo clone or
  via a protocol handler

## Medium - \$50 - $100

Medium severity issues allow an attacker to read or modify limited amounts of
data that they are not authorized to access.
They generally grant access to less sensitive information than high
severity issues.

### For example:

* Disclosing the title of issues in private repositories, which should be be inaccessible
* Injecting attacker controlled content into BlockMesh Network.com (XSS) but not bypassing CSP or executing sensitive
* Actions with another user’s session
* Bypassing CSRF validation for low risk actions, such as starring a repository or unsubscribing from a mailing list
* Code execution in a client app that requires minimal, expected user interaction, such as performing actions on a
* Repository or with a package that a user would not expect to lead to code execution
* Package integrity compromise, i.e., downloading a package that does not match the integrity as defined in
  package-lock.json

## Low - \$10 - \$50

Low severity issues allow an attacker to access extremely limited amounts of data.
They may violate an expectation for how something is intended to work but
allow nearly no escalation of privilege or ability to trigger unintended behavior by an attacker.

### For example:

* Signing up arbitrary users for access to an “early access feature” without their consent
* Creating an issue comment that bypasses our image proxying filter by providing a malformed URL
* Triggering verbose or debug error pages without proof of exploitability or obtaining sensitive information
* Triggering application exceptions that could affect many users
* Injecting JavaScript event handlers into links, etc., that are mitigated by CSP on BlockMesh Network.com
* Disclosing the existence of private packages on npm that should be inaccessible, e.g., through error messages (but not
* Through timing attacks, which are ineligible)
* Novel supply chain vulnerabilities that affect a BlockMesh Network product but are not solely limited to that product
* Credentials such as those from the .npmrc file or from BlockMesh Network Enterprise Server being leaked in logs