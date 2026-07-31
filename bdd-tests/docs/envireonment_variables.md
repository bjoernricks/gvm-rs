# BDD Test Environment Variables

All BDD test variables use the `GVMD_TEST_` prefix.

| Variable               | Default               | Description                                            |
| ---------------------- | --------------------- | ------------------------------------------------------ |
| `GVMD_TEST_SOCKET_PATH` | `/run/gvmd/gvmd.sock` | Path to the local gvmd Unix socket                     |
| `GVMD_TEST_USERNAME`    | `admin`               | Username used for authentication                       |
| `GVMD_TEST_PASSWORD`    | `admin`               | Password used for authentication                       |
| `GVMD_TEST_LOG_LEVEL`   | `info`                | Log level, such as `debug`, `info`, `warn`, or `error` |

## Example `.env`

```env
GVMD_TEST_SOCKET_PATH=/run/gvmd/gvmd.sock
GVMD_TEST_USERNAME=admin
GVMD_TEST_PASSWORD=admin
GVMD_TEST_LOG_LEVEL=debug
```

## Notes

* Environment variables override the default values.
* A local `.env` file can be used when running the tests.
