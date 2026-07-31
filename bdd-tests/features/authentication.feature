Feature: Authenticate with gvmd

    Scenario: Authenticate with valid credentials
        Given the local gvmd Unix socket is available
        When I authenticate with the configured credentials
        Then the authentication should succeed
        And an authenticated role should be returned
