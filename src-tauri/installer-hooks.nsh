!macro NSIS_HOOK_POSTINSTALL
  Delete "$DESKTOP\SpiritVale Overlay.lnk"
  CreateShortCut "$DESKTOP\SpiritVale Overlay.lnk" "$INSTDIR\spiritvale-overlay.exe" "" "$INSTDIR\spiritvale-overlay.exe" 0 SW_SHOWNORMAL "" "SpiritVale Overlay"
!macroend