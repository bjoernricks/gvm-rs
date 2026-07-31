Feature: Connect to the local GEA stack

  Scenario: Connect to gvmd through the Unix socket
    Given the local GEA Unix socket is available
    Then the GMP client should be connected