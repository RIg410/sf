
interface TelegramWebApp {
  initData: string;
  ready(): void;
  onEvent<T extends TelegramWebAppEventType>(
    eventType: T,
    eventHandler: TelegramWebAppEventHandlers[T]
  ): void;
  setHeaderColor(color: 'bg_color' | 'secondary_bg_color' | string): void;
  setBackgroundColor(color: 'bg_color' | 'secondary_bg_color' | string): void;
  HapticFeedback: HapticFeedback;
  close(): void;
  colorScheme: 'light' | 'dark';
  backgroundColor: string;
  isExpanded: boolean;

  expand(): void;
}

interface HapticFeedback {
  impactOccurred(style: 'light' | 'medium' | 'heavy' | 'rigid' | 'soft'): HapticFeedback;
  notificationOccurred(type: 'error' | 'success' | 'warning'): HapticFeedback;
  selectionChanged(): HapticFeedback;
}


interface Telegram {
  WebApp: TelegramWebApp;
}

type TelegramWebAppEventType =
  | 'viewportChanged'
  | 'themeChanged'
  | 'mainButtonClicked'
  | 'backButtonClicked'
  | 'settingsButtonClicked'
  | 'invoiceClosed'
  | 'popupClosed';

interface TelegramWebAppEventHandlers {
  viewportChanged: (params: { isStateStable: boolean }) => void;
  themeChanged: () => void;
  mainButtonClicked: () => void;
  backButtonClicked: () => void;
  settingsButtonClicked: () => void;
  invoiceClosed: (params: { url: string, status: 'paid' | 'cancelled' | 'failed' | 'pending' }) => void;
  popupClosed: (params: { button_id?: string }) => void;
}


export function tg_init() {
  const telegram = getTelegram();
  telegram.WebApp.ready();
  console.log("tg init");

  telegram.WebApp.onEvent('themeChanged', function () {
    document.documentElement.className = telegram.WebApp.colorScheme;
  });

  telegram.WebApp.setHeaderColor('secondary_bg_color');
  setViewportData();

  telegram.WebApp.onEvent('viewportChanged', setViewportData);

  telegram.WebApp.onEvent('themeChanged', function () {
    document.body.setAttribute('style', '--bg-color:' + telegram.WebApp.backgroundColor);
  });
}

function setViewportData() {
  const telegram = getTelegram();
  if (!telegram.WebApp.isExpanded) {
    telegram.WebApp.expand();
  }
}

function getTelegram(): Telegram {
  return (window as any).Telegram;
}


export function is_telegram_context(): boolean {
  return initData() !== '';
}

export function initData(): string {
  const telegram = getTelegram();
  const data = telegram.WebApp.initData;
  return data;
}

export function close() {
  const telegram = getTelegram();
  telegram.WebApp.close();
}

export function selectionChanged() {
  const telegram = getTelegram();
  telegram.WebApp.HapticFeedback.selectionChanged();
}

export function impactOccurred(style: 'light' | 'medium' | 'heavy' | 'rigid' | 'soft') {
  const telegram = getTelegram();
  telegram.WebApp.HapticFeedback.impactOccurred(style);
}
