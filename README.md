# MQRT

Small MQTT router. Allows creating multiple inputs/outputs and run action when input triggers.

### Features

- multi-(input/output)
- multiple actions tied to the same trigger
- JavaScript filtering (run JS to check if message contains what you are interested in)
- JavaScript payload builder (run JS to build output message based on input message)

### Configuration (_incomplete_):

```toml
# Define some input (name - "_" can be anything)
[input._]
type = "mqtt"
host = "127.0.0.1"
port = 1883

# ... with a number of triggers
[input._.trigger.single_click__hall_entrance_switch]
topic = 'zigbee2mqtt/hall_entrance_switch'
filter = { type = "json", field = "action", exact = "single_left" }
# ----- Note, that filter can also pass everything (default)
# filter = { type = 'no_filter' }
# ----- drop everything
# filter = { type = 'drop_all' }
# ----- check with JavaScript
# filter = { type = 'js', code = '''
# return JSON.parse(payload)['temperature'] > 20
# '''}

##############################
##############################

# Define some output (name - "_" can be anything)
[output._]
type = "mqtt"
host = "127.0.0.1"
port = 1883

# ... with a number of actions
[output._.action.toggle_hall_light]
topic = 'zigbee2mqtt/hall_light_main'
payload = { type = 'static', data = '{"action": "toggle"}' }
# ----- Note, that payload can be dropped
# payload = { type = 'drop' }
# ----- passthrough (which is default)
# payload = { type = 'passthrough' }
# ----- of JavaScript code (consider the performance impact in this case)
# payload = { type = 'js', code = '''
#   let x = JSON.parse(payload); x.volume += 1; return JSON.stringify(x);
# ''' }
# ----- or
# payload = { type = 'js', code = 'payload.split(",")[0]' }

##############################
##############################

# Now, specify which action should be executed on trigger
[[handler]]
on = { input = "_", trigger = "single_click__hall_entrance_switch" }
do = { output = "_", action = "toggle_hall_light" }
```
