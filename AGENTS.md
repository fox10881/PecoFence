# PecoFence Agent Rules

## DeepSeek consultation (mandatory)

For any non-trivial architectural change, Windows internals / COM / DirectX /
shell bug, or multi-step fix:

1. **Consult `deepseek v4 pro` BEFORE finalizing code.** Present the root-cause
   hypothesis and the proposed implementation to the `deepseek-v4-pro` MCP
   (`deepseek_consult` tool) and ask specifically for WMF / DirectComposition /
   shell best-practice review. Skipping the consult is not acceptable — it has
   repeatedly caught wrong assumptions (e.g. IDCompositionTarget retargeting).
2. **Critically evaluate the response.** Accept valid edge-case catches and
   corrections; reject suggestions that conflict with PecoFence's actual
   architecture or proven behavior. The MCP verdict is advisory, not
   authoritative.
3. **Attribute in every final summary.** State explicitly which parts of the
   solution came from the agent's own reasoning and which were suggested or
   corrected by `deepseek v4 pro`.
