; Ganchos del instalador NSIS de tuna-fonts (bundle.windows.nsis.installerHooks).
;
; 1. Se instala en %LOCALAPPDATA%\Programs\tuna-fonts (la carpeta conocida
;    FOLDERID_UserProgramFiles), no en %LOCALAPPDATA%\tuna-fonts como la 0.1.0.
; 2. Migración desde la 0.1.0: si está instalada en la ruta vieja, se quita en
;    silencio (el desinstalador en silencio no borra datos) y se conservan todos
;    los datos de %APPDATA%\es.atunfrito.tunafonts: ajustes, favoritas,
;    colecciones, atajos, carpetas vigiladas y las fuentes desactivadas.
;
; Tauri pone su ruta por defecto (o la de la instalación anterior) en .onInit;
; aquí se corrige en .onGUIInit, antes de enseñar la página de la carpeta, y otra
; vez al empezar a copiar, para las instalaciones en silencio (/S), que no tienen
; interfaz.

; El nombre a mano: estos ganchos se incluyen antes de que Tauri defina PRODUCTNAME.
!define TF_NOMBRE "tuna-fonts"
!define TF_DATOS "$APPDATA\es.atunfrito.tunafonts"
!define TF_COPIA "$TEMP\tuna-fonts-migracion"

Var TfVieja
Var TfEscritorio

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
    System::Call 'ole32::CoTaskMemFree(p r1)'
  ${EndIf}
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

!define MUI_CUSTOMFUNCTION_GUIINIT TfAlAbrir
Function TfAlAbrir
  !insertmacro TF_CORREGIR_CARPETA
  ; Red de seguridad: si hay una 0.1.0, copia de los datos antes de que su
  ; desinstalador pueda llegar a ejecutarse (por si alguien marca «borrar datos»).
  ${If} ${FileExists} "$LOCALAPPDATA\${TF_NOMBRE}\uninstall.exe"
  ${AndIf} ${FileExists} "${TF_DATOS}\*.*"
    RMDir /r "${TF_COPIA}"
    CreateDirectory "${TF_COPIA}"
    CopyFiles /SILENT "${TF_DATOS}\*.*" "${TF_COPIA}"
  ${EndIf}
FunctionEnd

!macro NSIS_HOOK_PREINSTALL
  !insertmacro TF_CORREGIR_CARPETA
  SetOutPath $INSTDIR

  ; La 0.1.0 sigue en la ruta vieja: se desinstala en silencio (deja los datos).
  StrCpy $TfVieja "$LOCALAPPDATA\${TF_NOMBRE}"
  StrCpy $TfEscritorio 0
  ${If} $INSTDIR != $TfVieja
  ${AndIf} ${FileExists} "$TfVieja\uninstall.exe"
    ${If} ${FileExists} "$DESKTOP\${TF_NOMBRE}.lnk"
      StrCpy $TfEscritorio 1
    ${EndIf}
    !insertmacro CheckIfAppIsRunning "$TfVieja\${MAINBINARYNAME}.exe" "${TF_NOMBRE}"
    DetailPrint "tuna-fonts 0.1.0 -> $INSTDIR"
    ExecWait '"$TfVieja\uninstall.exe" /S _?=$TfVieja'
    Delete "$TfVieja\uninstall.exe"
    RMDir "$TfVieja"
  ${EndIf}
!macroend

!macro NSIS_HOOK_POSTINSTALL
  ; Si los datos se borraron al quitar la versión vieja, vuelven de la copia.
  ${If} ${FileExists} "${TF_COPIA}\*.*"
    ${IfNot} ${FileExists} "${TF_DATOS}\*.*"
      CreateDirectory "${TF_DATOS}"
      CopyFiles /SILENT "${TF_COPIA}\*.*" "${TF_DATOS}"
    ${EndIf}
    RMDir /r "${TF_COPIA}"
  ${EndIf}
  ; El acceso directo del escritorio que tenía la 0.1.0, apuntando a la nueva.
  ${If} $TfEscritorio = 1
    CreateShortcut "$DESKTOP\${TF_NOMBRE}.lnk" "$INSTDIR\${MAINBINARYNAME}.exe"
  ${EndIf}
!macroend
