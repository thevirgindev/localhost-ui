// Local profile and notification management for Crunchyroll Anime

export interface UserProfile {
  id: string;
  name: string;
  avatar: string;
  color: string;
  isKid: boolean;
  createdAt: string;
}

export interface AiringNotification {
  id: string;
  animeId: number;
  animeTitle: string;
  episodeNumber: number;
  cover: string;
  message: string;
  timeAgo: string;
  read: boolean;
}

export interface AppPreferences {
  theme: "dark" | "oled" | "charcoal" | "light" | "navy" | "cyber";
  videoQuality: "auto" | "1080p" | "720p" | "4k";
  autoplay: boolean;
  autoSkipIntro: boolean;
  audioLanguage: "ja" | "en" | "es";
  subtitleLanguage: "en" | "es" | "pt" | "de" | "fr";
  subtitleColor: "#ffffff" | "#ffe600" | "#00ffff";
  subtitleSize: "small" | "medium" | "large";
  subtitleBg: "none" | "dark" | "outline";
  airingAlerts: boolean;
  accentColor: string;
  radius: string; // "0" | "2" | "4" | "6" | "8" | "12" | "16"
  fontFamily: string; // "Overpass" | "Inter" | "Poppins" | "Plus Jakarta Sans" | "Outfit" | "Roboto Mono" | "Fira Code" | "System Sans"
  cardStyle: "bordered" | "flat" | "elevated" | "glass";
  density: "compact" | "comfortable" | "spacious";
  cacheMinutes: number;
}

export interface CustomThemePreset {
  id: string;
  name: string;
  theme: AppPreferences["theme"];
  accentColor: string;
  radius: string;
  fontFamily: string;
  cardStyle: AppPreferences["cardStyle"];
}

const DEFAULT_PROFILES: UserProfile[] = [
  {
    id: "p_luci",
    name: "Luci",
    avatar: "https://images.unsplash.com/photo-1578632767115-351597cf2477?w=160&auto=format&fit=crop&q=80",
    color: "#a855f7",
    isKid: false,
    createdAt: "2026-01-01",
  },
  {
    id: "p_eren",
    name: "Survey Corps",
    avatar: "https://images.unsplash.com/photo-1534447677768-be436bb09401?w=160&auto=format&fit=crop&q=80",
    color: "#3b82f6",
    isKid: false,
    createdAt: "2026-02-15",
  },
  {
    id: "p_guest",
    name: "Guest",
    avatar: "https://images.unsplash.com/photo-1618005182384-a83a8bd57fbe?w=160&auto=format&fit=crop&q=80",
    color: "#10b981",
    isKid: true,
    createdAt: "2026-03-01",
  },
];

const DEFAULT_NOTIFICATIONS: AiringNotification[] = [
  {
    id: "notif_1",
    animeId: 21,
    animeTitle: "One Piece",
    episodeNumber: 1115,
    cover: "https://images.unsplash.com/photo-1607604276583-eef5d076aa5f?w=300&auto=format&fit=crop&q=80",
    message: "Episode 1115 is now streaming with English Subtitles!",
    timeAgo: "2 hours ago",
    read: false,
  },
  {
    id: "notif_2",
    animeId: 101922,
    animeTitle: "Demon Slayer",
    episodeNumber: 8,
    cover: "https://images.unsplash.com/photo-1579783902614-a3fb3927b675?w=300&auto=format&fit=crop&q=80",
    message: "Hashira Training Arc - English Dub premiere is live!",
    timeAgo: "5 hours ago",
    read: false,
  },
  {
    id: "notif_3",
    animeId: 113415,
    animeTitle: "Jujutsu Kaisen",
    episodeNumber: 24,
    cover: "https://images.unsplash.com/photo-1618005182384-a83a8bd57fbe?w=300&auto=format&fit=crop&q=80",
    message: "Shibuya Incident - Uncut Blu-Ray audio mix now available.",
    timeAgo: "1 day ago",
    read: false,
  },
  {
    id: "notif_4",
    animeId: 16498,
    animeTitle: "Attack on Titan",
    episodeNumber: 25,
    cover: "https://images.unsplash.com/photo-1518709268805-4e9042af9f23?w=300&auto=format&fit=crop&q=80",
    message: "Final Season Complete Colossal Edition added to library.",
    timeAgo: "2 days ago",
    read: true,
  },
  {
    id: "notif_5",
    animeId: 127230,
    animeTitle: "Chainsaw Man",
    episodeNumber: 12,
    cover: "https://images.unsplash.com/photo-1550684848-fac1c5b4e853?w=300&auto=format&fit=crop&q=80",
    message: "Reze Arc special cinematic preview added.",
    timeAgo: "3 days ago",
    read: true,
  },
];

const DEFAULT_PREFS: AppPreferences = {
  theme: "dark",
  videoQuality: "1080p",
  autoplay: true,
  autoSkipIntro: true,
  audioLanguage: "ja",
  subtitleLanguage: "en",
  subtitleColor: "#ffffff",
  subtitleSize: "medium",
  subtitleBg: "none",
  airingAlerts: true,
  accentColor: "#a855f7",
  radius: "4",
  fontFamily: "Overpass",
  cardStyle: "bordered",
  density: "comfortable",
  cacheMinutes: 30,
};

export const BUILT_IN_PRESETS: CustomThemePreset[] = [
  {
    id: "preset_luci_classic",
    name: "Luci Monochrome (Default)",
    theme: "dark",
    accentColor: "#a855f7",
    radius: "4",
    fontFamily: "Overpass",
    cardStyle: "bordered",
  },
  {
    id: "preset_oled_sharp",
    name: "OLED Pitch Sharp",
    theme: "oled",
    accentColor: "#00d2ff",
    radius: "0",
    fontFamily: "Fira Code",
    cardStyle: "bordered",
  },
  {
    id: "preset_clean_light",
    name: "Clean Modern Light",
    theme: "light",
    accentColor: "#3b82f6",
    radius: "4",
    fontFamily: "Inter",
    cardStyle: "bordered",
  },
  {
    id: "preset_cyber_neon",
    name: "Cyberpunk Neon",
    theme: "cyber",
    accentColor: "#8b5cf6",
    radius: "0",
    fontFamily: "Outfit",
    cardStyle: "glass",
  },
  {
    id: "preset_charcoal_emerald",
    name: "Charcoal Emerald",
    theme: "charcoal",
    accentColor: "#10b981",
    radius: "4",
    fontFamily: "Plus Jakarta Sans",
    cardStyle: "bordered",
  },
  {
    id: "preset_midnight_crimson",
    name: "Midnight Crimson",
    theme: "navy",
    accentColor: "#ef4444",
    radius: "2",
    fontFamily: "Poppins",
    cardStyle: "bordered",
  },
];

export function applyAppearanceToDocument(prefs: AppPreferences) {
  if (typeof document === "undefined") return;
  const root = document.documentElement;

  // Set theme attribute
  root.setAttribute("data-theme", prefs.theme || "dark");

  // Accent Colors
  const accent = prefs.accentColor || "#a855f7";
  root.style.setProperty("--accent", accent);

  // Compute hover & dim shades
  root.style.setProperty("--accent-hover", adjustColor(accent, 15));
  root.style.setProperty("--accent-dim", hexToRgba(accent, 0.16));

  // Corner radius (0px, 2px, 4px, 6px, 8px, 12px, 16px)
  const radNum = parseInt(prefs.radius || "4", 10);
  root.style.setProperty("--radius", `${radNum}px`);
  root.style.setProperty("--radius-sm", `${Math.max(0, radNum - 2)}px`);
  root.style.setProperty("--radius-lg", `${radNum + 4}px`);
  root.style.setProperty("--radius-btn", radNum === 0 ? "0px" : `${radNum}px`);

  // Fonts
  const fontMap: Record<string, string> = {
    Overpass: '"Overpass", "Inter", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
    Inter: '"Inter", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
    Poppins: '"Poppins", "Inter", -apple-system, BlinkMacSystemFont, sans-serif',
    "Plus Jakarta Sans": '"Plus Jakarta Sans", "Inter", -apple-system, BlinkMacSystemFont, sans-serif',
    Outfit: '"Outfit", "Inter", -apple-system, BlinkMacSystemFont, sans-serif',
    "Roboto Mono": '"Roboto Mono", monospace',
    "Fira Code": '"Fira Code", monospace',
    "System Sans": '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif',
  };
  root.style.setProperty("--font", fontMap[prefs.fontFamily] || fontMap.Overpass);
}

function hexToRgba(hex: string, alpha: number): string {
  let c = hex.replace("#", "");
  if (c.length === 3) {
    c = c.split("").map((x) => x + x).join("");
  }
  const num = parseInt(c, 16);
  if (isNaN(num)) return `rgba(168, 85, 247, ${alpha})`;
  const r = (num >> 16) & 255;
  const g = (num >> 8) & 255;
  const b = num & 255;
  return `rgba(${r}, ${g}, ${b}, ${alpha})`;
}

function adjustColor(hex: string, amount: number): string {
  let c = hex.replace("#", "");
  if (c.length === 3) {
    c = c.split("").map((x) => x + x).join("");
  }
  const num = parseInt(c, 16);
  if (isNaN(num)) return hex;
  const r = Math.min(255, Math.max(0, ((num >> 16) & 255) + amount));
  const g = Math.min(255, Math.max(0, ((num >> 8) & 255) + amount));
  const b = Math.min(255, Math.max(0, (num & 255) + amount));
  return `#${((1 << 24) + (r << 16) + (g << 8) + b).toString(16).slice(1)}`;
}

// Global reactive signals for user store
class UserStoreState {
  profiles = $state<UserProfile[]>([]);
  activeProfile = $state<UserProfile>(DEFAULT_PROFILES[0]);
  notifications = $state<AiringNotification[]>([]);
  preferences = $state<AppPreferences>(DEFAULT_PREFS);
  customPresets = $state<CustomThemePreset[]>([]);

  get appearance(): AppPreferences {
    return this.preferences;
  }

  // Modals state
  showSearchModal = $state(false);
  showProfileModal = $state(false);
  showChangelogModal = $state(false);
  showSettingsModal = $state(false);

  constructor() {
    this.loadFromStorage();
  }

  loadFromStorage() {
    try {
      const storedProfiles = localStorage.getItem("luci_profiles");
      if (storedProfiles) {
        this.profiles = JSON.parse(storedProfiles);
      } else {
        this.profiles = DEFAULT_PROFILES;
        localStorage.setItem("luci_profiles", JSON.stringify(DEFAULT_PROFILES));
      }

      const activeId = localStorage.getItem("luci_active_profile_id");
      const found = this.profiles.find((p) => p.id === activeId);
      this.activeProfile = found || this.profiles[0] || DEFAULT_PROFILES[0];

      const storedNotifs = localStorage.getItem("luci_notifications");
      if (storedNotifs) {
        this.notifications = JSON.parse(storedNotifs);
      } else {
        this.notifications = DEFAULT_NOTIFICATIONS;
        localStorage.setItem("luci_notifications", JSON.stringify(DEFAULT_NOTIFICATIONS));
      }

      const storedPresets = localStorage.getItem("luci_custom_presets");
      if (storedPresets) {
        this.customPresets = JSON.parse(storedPresets);
      }

      const storedPrefs = localStorage.getItem("luci_preferences");
      if (storedPrefs) {
        this.preferences = { ...DEFAULT_PREFS, ...JSON.parse(storedPrefs) };
      }
      applyAppearanceToDocument(this.preferences);
    } catch {
      this.profiles = DEFAULT_PROFILES;
      this.activeProfile = DEFAULT_PROFILES[0];
      this.notifications = DEFAULT_NOTIFICATIONS;
      this.preferences = DEFAULT_PREFS;
      applyAppearanceToDocument(DEFAULT_PREFS);
    }
  }

  setActiveProfile(profile: UserProfile) {
    this.activeProfile = profile;
    localStorage.setItem("luci_active_profile_id", profile.id);
  }

  addProfile(name: string, avatar: string, color: string = "#a855f7", isKid: boolean = false) {
    const newProfile: UserProfile = {
      id: "p_" + Date.now(),
      name: name.trim() || "Anime Fan",
      avatar: avatar || DEFAULT_PROFILES[0].avatar,
      color,
      isKid,
      createdAt: new Date().toISOString().split("T")[0],
    };
    this.profiles = [...this.profiles, newProfile];
    this.setActiveProfile(newProfile);
    localStorage.setItem("luci_profiles", JSON.stringify(this.profiles));
  }

  deleteProfile(id: string) {
    if (this.profiles.length <= 1) return;
    this.profiles = this.profiles.filter((p) => p.id !== id);
    if (this.activeProfile.id === id) {
      this.setActiveProfile(this.profiles[0]);
    }
    localStorage.setItem("luci_profiles", JSON.stringify(this.profiles));
  }

  updateProfile(id: string, updates: Partial<UserProfile>) {
    this.profiles = this.profiles.map((p) => (p.id === id ? { ...p, ...updates } : p));
    if (this.activeProfile.id === id) {
      this.activeProfile = { ...this.activeProfile, ...updates };
    }
    localStorage.setItem("luci_profiles", JSON.stringify(this.profiles));
  }

  updatePreferences(updates: Partial<AppPreferences>) {
    this.preferences = { ...this.preferences, ...updates };
    applyAppearanceToDocument(this.preferences);
    localStorage.setItem("luci_preferences", JSON.stringify(this.preferences));
  }

  saveCustomPreset(name: string) {
    const newPreset: CustomThemePreset = {
      id: "preset_user_" + Date.now(),
      name: name.trim() || `Custom Preset ${this.customPresets.length + 1}`,
      theme: this.preferences.theme,
      accentColor: this.preferences.accentColor,
      radius: this.preferences.radius,
      fontFamily: this.preferences.fontFamily,
      cardStyle: this.preferences.cardStyle,
    };
    this.customPresets = [...this.customPresets, newPreset];
    localStorage.setItem("luci_custom_presets", JSON.stringify(this.customPresets));
  }

  deleteCustomPreset(id: string) {
    this.customPresets = this.customPresets.filter((p) => p.id !== id);
    localStorage.setItem("luci_custom_presets", JSON.stringify(this.customPresets));
  }

  applyPreset(preset: CustomThemePreset) {
    this.updatePreferences({
      theme: preset.theme,
      accentColor: preset.accentColor,
      radius: preset.radius,
      fontFamily: preset.fontFamily,
      cardStyle: preset.cardStyle,
    });
  }

  markNotificationRead(id: string) {
    this.notifications = this.notifications.map((n) => (n.id === id ? { ...n, read: true } : n));
    localStorage.setItem("luci_notifications", JSON.stringify(this.notifications));
  }

  markAllNotificationsRead() {
    this.notifications = this.notifications.map((n) => ({ ...n, read: true }));
    localStorage.setItem("luci_notifications", JSON.stringify(this.notifications));
  }

  clearNotifications() {
    this.notifications = [];
    localStorage.setItem("luci_notifications", JSON.stringify([]));
  }

  get unreadNotificationsCount() {
    return this.notifications.filter((n) => !n.read).length;
  }
}

export const userStore = new UserStoreState();
