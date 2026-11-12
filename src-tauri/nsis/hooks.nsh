; Ganchos del instalador NSIS de tuna-fonts (bundle.windows.nsis.installerHooks).
;
; 1. Se instala en %LOCALAPPDATA%\Programs\tuna-fonts (la carpeta conocida
;    FOLDERID_UserProgramFiles), no en %LOCALAPPDATA%\tuna-fonts como la 0.1.0.
; 2. Migración desde la 0.1.0, igual con ventana que en silencio (/S):
;    - se copia %APPDATA%\es.atunfrito.tunafonts antes de tocar nada y se
;      devuelve si al final faltara (ajustes, favoritas, colecciones, atajos,
;      carpetas vigiladas y las fuentes desactivadas);
;    - la 0.1.0 se quita con su propio desinstalador en modo /S /UPDATE: en ese
;      modo no borra accesos directos, no quita anclajes y nunca borra datos;
;    - los accesos directos que apuntaban a la 0.1.0 (menú Inicio, escritorio y
;      los anclados a la barra de tareas) se apuntan al .exe nuevo.
;
; La página «ya hay una versión instalada» de Tauri ejecutaría el desinstalador
; viejo con ventana y en modo normal (borra accesos y anclajes), así que cuando
; la instalación anterior es la 0.1.0 se esconde esa página: se aparta su
; UninstallString al abrir el instalador y se repone si se cancela.

; El nombre a mano: estos ganchos se incluyen antes de que Tauri defina PRODUCTNAME.
!define TF_NOMBRE "tuna-fonts"
!define TF_DATOS "$APPDATA\es.atunfrito.tunafonts"
!define TF_COPIA "$TEMP\tuna-fonts-migracion"
!define TF_CLAVE "Software\Microsoft\Windows\CurrentVersion\Uninstall\tuna-fonts"
!define TF_ANCLADOS "$APPDATA\Microsoft\Internet Explorer\Quick Launch\User Pinned\TaskBar"

Var TfVieja
Var TfApartado

; %LOCALAPPDATA%\Programs, pedida a Windows como carpeta conocida.
!macro TF_RUTA_PROGRAMAS SALIDA
  Push $0
  Push $1
  StrCpy ${SALIDA} ""
  ; KF_FLAG_CREATE (0x8000): la crea si aún no existe.
  System::Call 'shell32::SHGetKnownFolderPath(g "{5CD7AEE2-2219-4A67-B85D-6C9CE15660CB}", i 0x8000, p 0, *p .r1) i .r0'
  ${If} $0 = 0
    System::Call '*$1(&w${NSIS_MAX_STRLEN} .s)'
    Pop ${SALIDA}
  ${EndIf}
  System::Call 'ole32::CoTaskMemFree(p r1)'
  ${If} ${SALIDA} == ""
    StrCpy ${SALIDA} "$LOCALAPPDATA\Programs"
  ${EndIf}
  Pop $1
  Pop $0
!macroend

; Si la carpeta elegida es la de antes (la de la 0.1.0), se cambia por la nueva.
!macro TF_CORREGIR_CARPETA
  StrCpy $TfVieja "$LOCALAPPDATA\${TF_NOMBRE}"
  ${If} $INSTDIR == $TfVieja
  ${OrIf} $INSTDIR == "$TfVieja\"
    Push $2
    !insertmacro TF_RUTA_PROGRAMAS $2
    StrCpy $INSTDIR "$2\${TF_NOMBRE}"
    Pop $2
  ${EndIf}
!macroend

; Un acceso directo que apunta al .exe viejo pasa a apuntar al nuevo.
; Usa $0-$3 (los guarda quien la llama).
!macro TF_REAPUNTAR LNK
  ${If} ${FileExists} "${LNK}"
    !insertmacro IsShortcutTarget "${LNK}" "$TfVieja\${TF_NOMBRE}.exe"
    Pop $0
    ${If} $0 = 1
      !insertmacro SetShortcutTarget "${LNK}" "$INSTDIR\${TF_NOMBRE}.exe"
      DetailPrint "-> ${LNK}"
    ${EndIf}
  ${EndIf}
!macroend

!define MUI_CUSTOMFUNCTION_GUIINIT TfAlAbrir
Function TfAlAbrir
  !insertmacro TF_CORREGIR_CARPETA
  ; Hay una 0.1.0 en la ruta vieja: se esconde la página de reinstalar de Tauri
  ; (la migración la hace NSIS_HOOK_PREINSTALL).
  StrCpy $TfApartado ""
  ${If} ${FileExists} "$TfVieja\uninstall.exe"
    ReadRegStr $TfApartado HKCU "${TF_CLAVE}" "UninstallString"
    ${If} $TfApartado != ""
      DeleteRegValue HKCU "${TF_CLAVE}" "UninstallString"
    ${EndIf}
  ${EndIf}
FunctionEnd

; Si se cancela la instalación, la 0.1.0 vuelve a quedar como estaba en Aplicaciones.
!define MUI_CUSTOMFUNCTION_ABORT TfAlCancelar
Function TfAlCancelar
  ${If} $TfApartado != ""
    WriteRegStr HKCU "${TF_CLAVE}" "UninstallString" $TfApartado
    StrCpy $TfApartado ""
  ${EndIf}
FunctionEnd

!macro NSIS_HOOK_PREINSTALL
  !insertmacro TF_CORREGIR_CARPETA
  SetOutPath $INSTDIR

  StrCpy $TfVieja "$LOCALAPPDATA\${TF_NOMBRE}"
  ${If} $INSTDIR != $TfVieja
  ${AndIf} ${FileExists} "$TfVieja\uninstall.exe"
    !insertmacro CheckIfAppIsRunning "$TfVieja\${TF_NOMBRE}.exe" "${TF_NOMBRE}"

    ; Red de seguridad: copia de los datos antes de ejecutar nada de la versión vieja.
    ${If} ${FileExists} "${TF_DATOS}\*.*"
      RMDir /r "${TF_COPIA}"
      CreateDirectory "${TF_COPIA}"
      CopyFiles /SILENT "${TF_DATOS}\*.*" "${TF_COPIA}"
    ${EndIf}

    ; /UPDATE: deja accesos directos, anclajes y datos; solo quita sus archivos.
    DetailPrint "tuna-fonts 0.1.0 -> $INSTDIR"
    ExecWait '"$TfVieja\uninstall.exe" /S /UPDATE _?=$TfVieja'
    Delete "$TfVieja\uninstall.exe"
    RMDir "$TfVieja"
    StrCpy $TfApartado ""
  ${EndIf}
!macroend

!macro NSIS_HOOK_POSTINSTALL
  ; Si los datos se perdieron al quitar la versión vieja, vuelven de la copia.
  ${If} ${FileExists} "${TF_COPIA}\*.*"
    ${IfNot} ${FileExists} "${TF_DATOS}\*.*"
      CreateDirectory "${TF_DATOS}"
      CopyFiles /SILENT "${TF_COPIA}\*.*" "${TF_DATOS}"
    ${EndIf}
    RMDir /r "${TF_COPIA}"
  ${EndIf}

  ; Accesos directos de la 0.1.0 (escritorio, menú Inicio y barra de tareas)
  ; apuntando al .exe nuevo. Los anclados a la barra de tareas se miran todos,
  ; porque su nombre depende de desde dónde se anclaron.
  StrCpy $TfVieja "$LOCALAPPDATA\${TF_NOMBRE}"
  ${If} $INSTDIR != $TfVieja
    Push $0
    Push $1
    Push $2
    Push $3
    Push $4
    Push $5
    !insertmacro TF_REAPUNTAR "$DESKTOP\${TF_NOMBRE}.lnk"
    !insertmacro TF_REAPUNTAR "$SMPROGRAMS\${TF_NOMBRE}.lnk"
    FindFirst $4 $5 "${TF_ANCLADOS}\*.lnk"
    ${DoWhile} $5 != ""
      !insertmacro TF_REAPUNTAR "${TF_ANCLADOS}\$5"
      FindNext $4 $5
    ${Loop}
    FindClose $4
    Pop $5
    Pop $4
    Pop $3
    Pop $2
    Pop $1
    Pop $0
  ${EndIf}
!macroend
