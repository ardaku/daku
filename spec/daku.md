# Daku Custom Section

The Daku custom section is a WebAssembly custom section that must follow the
order of conventional sections

 1. `name`
 2. `producers`
 3. `target_features`
 4. `daku`

The base section without extensions is just a WebAssembly vector of portal IDs.

For details on the experimental nucleide extension see
<https://docs.rs/nucleide/latest/nucleide/#daku-daku>
