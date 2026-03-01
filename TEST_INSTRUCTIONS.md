# Test Instructions

## Please run these commands:

1. **Delete old logs:**
   ```bash
   rm ~/.local/share/web-radio/web-radio.log.*
   ```

2. **Run the app:**
   ```bash
   ./target/release/web-radio
   ```

3. **In the app:**
   - Wait 2 seconds for stations to load
   - Press Enter ONCE on a station
   - Wait 5 seconds
   - Press Q to quit

4. **Show me the logs:**
   ```bash
   cat ~/.local/share/web-radio/web-radio.log.*
   ```

## What I'm Looking For

The logs should now show one of these patterns:

### Pattern A: Blocked at count_click
```
INFO Playing station: X
INFO BEFORE count_click API call
(nothing after this - hanging here)
```

### Pattern B: Blocked at history add
```
INFO Playing station: X
INFO BEFORE count_click API call
INFO AFTER count_click API call
INFO BEFORE adding to history
(nothing after this - hanging here)
```

### Pattern C: Blocked at send command
```
INFO Playing station: X
INFO BEFORE count_click API call
INFO AFTER count_click API call
INFO BEFORE adding to history
INFO AFTER adding to history
INFO BEFORE sending play command
(nothing after this - hanging here)
```

### Pattern D: Working!
```
INFO Playing station: X
INFO BEFORE count_click API call
INFO AFTER count_click API call
INFO BEFORE adding to history
INFO AFTER adding to history
INFO BEFORE sending play command
INFO Play command sent successfully to player
INFO Received player command: Play(X)
INFO Processing Play command for: X
INFO Starting playback: X
```

This will tell us exactly where the code is getting stuck!
