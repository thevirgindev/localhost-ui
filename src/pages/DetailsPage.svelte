<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "../lib/api";
  import { reportError } from "../lib/ipc";
  import type { AnimeCard, EpisodeEntry, ProgressEntry } from "../lib/types";
  import { router } from "../lib/router";
  import { formatDesc } from "../lib/format";

  interface CharacterCard {
    id: number;
    name: string;
    image: string | null;
    role: string;
    voiceActor: string | null;
  }

  interface CommentItem {
    id: string;
    author: string;
    avatar: string;
    timeAgo: string;
    text: string;
    votes: number;
    isSpoiler: boolean;
    userVoted?: "up" | "down";
  }

  let { id }: { id: number } = $props();

  let card: AnimeCard | null = $state(null);
  let episodes: EpisodeEntry[] = $state([]);
  let characters: CharacterCard[] = $state([]);
  let inList = $state(false);
  let progress: ProgressEntry[] = $state([]);
  let loading = $state(true);
  let error = $state("");

  let activeSeason = $state("Season 1");
  let activeTab = $state<"info" | "episodes" | "characters" | "comments">("info");

  // Interactive Comments state (Image 2)
  let newCommentText = $state("");
  let isSpoiler = $state(false);
  let commentsList = $state<CommentItem[]>([
    {
      id: "c1",
      author: "Rliverr",
      avatar: "https://images.unsplash.com/photo-1534528741775-53994a69daeb?w=100&auto=format&fit=crop&q=80",
      timeAgo: "3hrs ago",
      text: "The animation in this arc is completely unmatched! Ufotable never ceases to amaze.",
      votes: 304,
      isSpoiler: false,
    },
    {
      id: "c2",
      author: "Kenzou_99",
      avatar: "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=100&auto=format&fit=crop&q=80",
      timeAgo: "6hrs ago",
      text: "That soundtrack during the climax gave me chills. 10/10 episode.",
      votes: 142,
      isSpoiler: false,
    },
    {
      id: "c3",
      author: "SakuraBlossom",
      avatar: "https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=100&auto=format&fit=crop&q=80",
      timeAgo: "1 day ago",
      text: "Can we talk about the character development? Best anime of the season without question!",
      votes: 89,
      isSpoiler: false,
    },
  ]);

  onMount(async () => {
    try {
      const c = await api.details(id);
      card = c;
      [inList, progress] = await Promise.all([api.inWatchlist(id), api.progress(id)]);
      const src = await api.episodes(id, c.title, c.titleEnglish);
      episodes = src.episodes;
      apiDetailsExtras(id);
    } catch (e) {
      error = String(e);
      reportError("DetailsPage.load", e);
    } finally {
      loading = false;
    }
  });

  async function apiDetailsExtras(animeId: number) {
    try {
      characters = await api.characters(animeId);
    } catch {
      // Optional characters
    }
  }

  async function toggleWatchlist() {
    if (!card) return;
    if (inList) {
      await api.removeFromWatchlist(id);
      inList = false;
    } else {
      await api.addToWatchlist(card, "planning");
      inList = true;
    }
  }

  function epProgress(n: number): ProgressEntry | undefined {
    return progress.find((p) => p.episode === n);
  }

  function firstUnwatched(): number {
    const watched = new Set(progress.filter((p) => p.completed).map((p) => p.episode));
    let n = 1;
    while (watched.has(n)) n++;
    return n;
  }

  const lastCompleted = $derived(
    progress.filter((p) => p.completed).reduce((max, p) => Math.max(max, p.episode), 0),
  );

  function submitComment(e: SubmitEvent) {
    e.preventDefault();
    if (!newCommentText.trim()) return;
    commentsList = [
      {
        id: "c_" + Date.now(),
        author: "Syzder",
        avatar: "https://images.unsplash.com/photo-1578632767115-351597cf2477?w=100&auto=format&fit=crop&q=80",
        timeAgo: "Just now",
        text: newCommentText.trim(),
        votes: 1,
        isSpoiler,
      },
      ...commentsList,
    ];
    newCommentText = "";
    isSpoiler = false;
  }

  function voteComment(c: CommentItem, dir: "up" | "down") {
    if (c.userVoted === dir) {
      c.votes += dir === "up" ? -1 : 1;
      c.userVoted = undefined;
    } else {
      if (c.userVoted) {
        c.votes += dir === "up" ? 2 : -2;
      } else {
        c.votes += dir === "up" ? 1 : -1;
      }
      c.userVoted = dir;
    }
    commentsList = [...commentsList];
  }

  let sortOrder = $state<"asc" | "desc">("asc");

  const voteCountFormatted = $derived.by(() => {
    if (!card) return "650";
    return String(Math.floor((card.averageScore ?? 85) * 12 + 250));
  });

  const seasonsList = $derived.by(() => {
    if (!card) return ["Season 1"];
    const eps = card.episodes ?? 12;
    if (eps > 36) return ["Season 1", "Season 2", "Season 3", "Season 4"];
    if (eps > 24) return ["Season 1", "Season 2", "Season 3"];
    if (eps > 12) return ["Season 1", "Season 2"];
    return ["Season 1"];
  });

  const seasonEpisodes = $derived.by(() => {
    let list: EpisodeEntry[] = [];
    const total = episodes.length;

    if (activeSeason === "Season 1") {
      list = episodes.slice(0, Math.min(25, total));
    } else if (activeSeason === "Season 2") {
      const s2 = episodes.slice(25, 37);
      list = s2.length > 0 ? s2 : episodes.slice(0, 12).map((e) => ({ ...e, number: e.number + 25 }));
    } else if (activeSeason === "Season 3") {
      const s3 = episodes.slice(37, 59);
      list = s3.length > 0 ? s3 : episodes.slice(0, 12).map((e) => ({ ...e, number: e.number + 37 }));
    } else {
      list = episodes;
    }

    if (sortOrder === "desc") {
      return [...list].reverse();
    }
    return list;
  });

  function getEpisodeSynopsis(c: AnimeCard | null, epNum: number): string {
    if (!c) return "No synopsis available.";
    if (c.id === 16498) {
      const aotSnips: Record<number, string> = {
        1: "After a century of peace behind massive walls, the Colossal Titan suddenly appears and breaches Wall Maria, shattering humanity's haven.",
        2: "Eren, Mikasa, and Armin escape onto an evacuation vessel bound for Wall Rose, vowing vengeance against all Titans.",
        3: "Eren and his fellow recruits join the 104th Training Corps, enduring instructor Keith Shadis's grueling evaluation.",
        4: "The cadets celebrate their graduation ceremony, preparing to select their military branches among the Scout, Garrison, or MP regiments.",
        5: "The Colossal Titan returns unexpectedly at Trost District, thrusting Eren's squad into their first lethal confrontation.",
        6: "Amid the catastrophe in Trost, Mikasa recalls the traumatic day Eren saved her from ruthless kidnappers.",
        7: "With gas supplies depleted, the remaining trainees confront despair until a rogue Titan turns on its own kind.",
        8: "Armin devises a tactical operation to reclaim the military headquarters, uncovering an astonishing secret.",
        9: "The military turns their cannons on Eren, who struggles to recall the mysterious key given by his father.",
        10: "Armin risks his life making an impassioned speech to the garrison troops, pleading for Eren's humanity.",
      };
      return aotSnips[epNum] || `Eren and the Scout Regiment execute vital maneuvers against Titans beyond the defensive perimeter.`;
    }

    if (c.id === 101922) {
      const dsSnips: Record<number, string> = {
        1: "Tanjiro returns from town to discover his family massacred, and his younger sister Nezuko transformed into a demon.",
        2: "Guided by Giyu Tomioka, Tanjiro and Nezuko travel toward Mount Sagiri to seek out Trainer Sakonji Urokodaki.",
        3: "Tanjiro trains relentlessly on Mount Sagiri, confronting the spirits of former disciples Sabito and Makomo.",
        4: "Tanjiro journeys to Mount Fujikasane to undertake Final Selection, encountering the grotesque Hand Demon.",
      };
      return dsSnips[epNum] || `Tanjiro confronts dangerous demons while learning the foundational forms of Water Breathing.`;
    }

    if (c.id === 113415) {
      const jjkSnips: Record<number, string> = {
        1: "Yuji Itadori encounters Megumi Fushiguro and ingests Ryomen Sukuna's cursed finger to save his school friends.",
        2: "Satoru Gojo takes Yuji to Jujutsu High in Tokyo, presenting him before Principal Yaga for admission.",
        3: "Yuji and Megumi rendezvous with third first-year student Nobara Kugisaki in Roppongi on a cursed curse mission.",
        4: "The first-years are deployed to an ominous juvenile detention center where a Special Grade curse womb has manifested.",
      };
      return jjkSnips[epNum] || `Jujutsu sorcerers battle malevolent curses threatening modern society.`;
    }

    return `${c.title} • Episode ${epNum}. Follow the journey as new challenges and dramatic events unfold.`;
  }

  const infoRows = $derived.by(() => {
    const c = card;
    if (!c) return [] as { label: string; value: string }[];
    const rows: { label: string; value: string }[] = [];
    if (c.format) rows.push({ label: "Type", value: c.format === "MOVIE" ? "Movie" : "Series" });
    if (c.episodes) rows.push({ label: "Episodes", value: String(c.episodes) });
    if (c.duration) rows.push({ label: "Duration", value: `${c.duration} min` });
    if (c.status) rows.push({ label: "Status", value: c.status === "RELEASING" ? "Currently Airing" : c.status });
    if (c.season || c.seasonYear) rows.push({ label: "Season", value: [c.season, c.seasonYear].filter(Boolean).join(" ") });
    if (c.studio) rows.push({ label: "Studio", value: c.studio });
    if (c.averageScore) rows.push({ label: "Score", value: `${c.averageScore}%` });
    return rows;
  });
</script>

<div class="details-page">
  {#if loading}
    <div class="spinner"></div>
  {:else if error || !card}
    <div class="page">
      <div class="error-box">
        <span>{error || "Anime details not found"}</span>
        <button class="btn secondary" onclick={() => history.back()}>Back</button>
      </div>
    </div>
  {:else}
    <!-- Hero Banner with full bleed under transparent navbar (Image 1) -->
    <section
      class="banner-hero"
      data-anime-id={card.id}
      data-anime-title={card.title}
      data-anime-cover={card.cover || card.banner || ""}
    >
      <div
        class="banner-bg"
        style={card.banner || card.cover ? `background-image: url('${card.banner || card.cover}')` : ""}
      ></div>
      <div class="banner-shade"></div>

      <div class="banner-content">
        <div class="meta-tag-row">
          <span class="type-badge">{card.format === "MOVIE" ? "Movie" : "Series"}</span>
          <span class="sub-badge">Sub | Dub</span>
        </div>
        <h1 class="anime-title">{card.title}</h1>

        <p class="synopsis-hero">
          {formatDesc(card.description).slice(0, 220)}{formatDesc(card.description).length > 220 ? "…" : ""}
        </p>

        <!-- Metadata Badges in Hero: Year • Studio • Episodes • Rating • Subtitles -->
        <div class="hero-meta-strip">
          <span class="meta-item">{card.startDate?.slice(0, 4) || card.seasonYear || "2024"}</span>
          <span class="meta-sep">•</span>
          <span class="meta-item">{card.studio || "ufotable"}</span>
          <span class="meta-sep">•</span>
          <span class="meta-item">{card.episodes ?? 12} Episodes</span>
          <span class="meta-sep">•</span>
          <span class="rating-badge">PG-13</span>
          <span class="meta-sep">•</span>
          <span class="meta-subs">Subtitles: 🇺🇸 🇲🇽 🇧🇷 🇯🇵 🇪🇸 🇩🇪</span>
        </div>

        <!-- 5 Stars + Vote count -->
        <div class="rating-strip">
          <div class="stars">★★★★★</div>
          <span class="votes-count">{voteCountFormatted} votes</span>
        </div>

        <!-- Action buttons: Crisp Crunchyroll buttons -->
        <div class="hero-actions">
          <button
            class="cr-orange-pill"
            onclick={() => router.navigate({ name: "watch", id, episode: firstUnwatched() })}
          >
            <svg viewBox="0 0 24 24" fill="currentColor" class="play-icon">
              <path d="M8 5v14l11-7z" />
            </svg>
            <span>{lastCompleted > 0 ? "Continue Watching" : "Start Watching"}</span>
          </button>

          <button class="cr-outline-pill" onclick={toggleWatchlist}>
            {#if inList}
              <svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
                <path d="M9 16.2L4.8 12l-1.4 1.4L9 19 21 7l-1.4-1.4L9 16.2z" />
              </svg>
              <span>Following</span>
            {:else}
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" width="16" height="16">
                <line x1="12" y1="5" x2="12" y2="19" />
                <line x1="5" y1="12" x2="19" y2="12" />
              </svg>
              <span>Follow Anime</span>
            {/if}
          </button>
        </div>
      </div>
    </section>

    <!-- Quick Season Switcher Bar -->
    {#if seasonsList.length > 1}
      <div class="seasons-bar-container">
        <div class="seasons-bar-inner">
          <span class="seasons-bar-title">Seasons:</span>
          <div class="seasons-chips-row">
            {#each seasonsList as s}
              <button
                class="season-chip"
                class:active={activeSeason === s}
                onclick={() => {
                  activeSeason = s;
                  activeTab = "episodes";
                }}
              >
                <span class="season-chip-name">{s}</span>
                <span class="season-chip-eps">({s === "Season 1" ? Math.min(25, episodes.length) : 12} Eps)</span>
              </button>
            {/each}
          </div>
        </div>
      </div>
    {/if}

    <!-- Clean Tab Navigation Bar: About first, followed by Episodes, Characters, and Comments -->
    <div class="season-nav-wrap">
      <div class="season-nav">
        <button
          class="season-tab"
          class:active={activeTab === "info"}
          onclick={() => (activeTab = "info")}
        >
          About
        </button>

        <button
          class="season-tab"
          class:active={activeTab === "episodes"}
          onclick={() => (activeTab = "episodes")}
        >
          Episodes ({episodes.length})
        </button>

        <button
          class="season-tab"
          class:active={activeTab === "characters"}
          onclick={() => (activeTab = "characters")}
        >
          Characters
        </button>

        <button
          class="season-tab"
          class:active={activeTab === "comments"}
          onclick={() => (activeTab = "comments")}
        >
          Comments ({commentsList.length})
        </button>
      </div>
    </div>

    <!-- Content Area: About OR Episodes Grid OR Characters OR Comments -->
    <div class="details-body">
      {#if activeTab === "info"}
        <section class="info-layout">
          <div class="info-main">
            <!-- Synopsis Block -->
            <div class="about-block">
              <h3 class="block-title">Synopsis</h3>
              <p class="synopsis">{formatDesc(card.description) || "No synopsis available."}</p>
            </div>

            <!-- Alternative Titles -->
            <div class="about-block titles-block">
              <h3 class="block-title">Alternative Titles</h3>
              <div class="alt-titles-grid">
                <div class="alt-title-row">
                  <span class="alt-label">English:</span>
                  <span class="alt-val">{card.title}</span>
                </div>
                <div class="alt-title-row">
                  <span class="alt-label">Romaji:</span>
                  <span class="alt-val">{card.title}</span>
                </div>
                <div class="alt-title-row">
                  <span class="alt-label">Native (日本語):</span>
                  <span class="alt-val">{card.studio ? `${card.title} (${card.studio})` : card.title}</span>
                </div>
              </div>
            </div>

            <!-- Related Seasons & Sequels -->
            {#if seasonsList.length > 1}
              <div class="about-block">
                <h3 class="block-title">Related Seasons & Franchise</h3>
                <div class="related-seasons-grid">
                  {#each seasonsList as s, i}
                    <div class="season-card-rel" class:current={activeSeason === s}>
                      <div class="rel-season-num">S{i + 1}</div>
                      <div class="rel-season-info">
                        <div class="rel-season-name">{card.title} - {s}</div>
                        <div class="rel-season-sub">{s === "Season 1" ? Math.min(25, episodes.length) : 12} Episodes • {card.seasonYear || "2024"}</div>
                      </div>
                      <button
                        class="btn secondary sm rel-action-btn"
                        onclick={() => {
                          activeSeason = s;
                          activeTab = "episodes";
                        }}
                      >
                        {activeSeason === s ? "View Active" : "Go to Season"}
                      </button>
                    </div>
                  {/each}
                </div>
              </div>
            {/if}

            <!-- Key Cast Preview -->
            {#if characters.length > 0}
              <div class="about-block">
                <div class="block-header-flex">
                  <h3 class="block-title">Main Characters & Cast</h3>
                  <button class="view-all-link" onclick={() => (activeTab = "characters")}>
                    View All ({characters.length}) →
                  </button>
                </div>
                <div class="cast-preview-row">
                  {#each characters.slice(0, 4) as char}
                    <div class="cast-preview-card">
                      {#if char.image}
                        <img src={char.image} alt={char.name} class="cast-img" />
                      {:else}
                        <div class="cast-placeholder">{char.name[0]}</div>
                      {/if}
                      <div class="cast-name">{char.name}</div>
                      <div class="cast-role">{char.role}</div>
                    </div>
                  {/each}
                </div>
              </div>
            {/if}

            <!-- Genres -->
            {#if card.genres.length}
              <div class="about-block">
                <h3 class="block-title">Genres & Themes</h3>
                <div class="genre-pills">
                  {#each card.genres as g}
                    <a class="genre-pill" href={`#/browse?q=${encodeURIComponent(g)}`}>{g}</a>
                  {/each}
                </div>
              </div>
            {/if}
          </div>

          <aside class="info-side">
            <h3 class="block-title">Details & Specifications</h3>
            {#each infoRows as row}
              <div class="info-row">
                <span class="info-label">{row.label}</span>
                <span class="info-value">{row.value}</span>
              </div>
            {/each}
            <div class="info-row">
              <span class="info-label">Audio</span>
              <span class="info-value">Japanese, English (Dub)</span>
            </div>
            <div class="info-row">
              <span class="info-label">Subtitles</span>
              <span class="info-value">English, Spanish, Portuguese</span>
            </div>
            <div class="info-row">
              <span class="info-label">Video Format</span>
              <span class="info-value">1080p FHD (H.264 / AAC)</span>
            </div>
          </aside>
        </section>
      {:else if activeTab === "episodes"}
        <!-- Episodes Section (Crunchyroll Episode Grid Layout) -->
        <section class="episodes-section">
          <div class="section-top">
            <div class="heading-left">
              {#if seasonsList.length > 1}
                <div class="season-selector-pills">
                  {#each seasonsList as s}
                    <button
                      class="season-pill-btn"
                      class:active={activeSeason === s}
                      onclick={() => (activeSeason = s)}
                    >
                      {s}
                    </button>
                  {/each}
                </div>
              {:else}
                <h2 class="ep-section-heading">{activeSeason} Episodes</h2>
              {/if}
              <span class="ep-count-pill">{seasonEpisodes.length} Episodes</span>
            </div>

            <div class="ep-toolbar-right">
              <!-- Sort toggle: Icon only, larger, no text -->
              <button
                class="ep-tool-btn icon-only-sort"
                title={sortOrder === "asc" ? "Sort: 1 to N (Click to reverse)" : "Sort: N to 1 (Click to reverse)"}
                onclick={() => (sortOrder = sortOrder === "asc" ? "desc" : "asc")}
                aria-label="Sort episodes"
              >
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" width="20" height="20">
                  <path d="M3 6h18M6 12h12M10 18h4" />
                </svg>
              </button>
            </div>
          </div>

          {#if seasonEpisodes.length === 0}
            <div class="empty-state">
              <p>No episodes found for this season.</p>
            </div>
          {:else}
            <!-- Episode Grid -->
            <div class="episodes-grid">
              {#each seasonEpisodes as ep (ep.number)}
                {@const prog = epProgress(ep.number)}
                {@const percent = prog ? Math.round((prog.positionSec / Math.max(1, prog.durationSec)) * 100) : 0}
                <div
                  class="ep-card"
                  data-episode-number={ep.number}
                  data-anime-id={id}
                  data-episode-title={ep.title}
                >
                  <div
                    class="ep-thumb"
                    onclick={() => router.navigate({ name: "watch", id, episode: ep.number })}
                    role="button"
                    tabindex="0"
                    onkeydown={(e) => e.key === "Enter" && router.navigate({ name: "watch", id, episode: ep.number })}
                  >
                    {#if card.banner || card.cover}
                      <img src={card.banner || card.cover} alt={ep.title} loading="lazy" />
                    {:else}
                      <div class="ep-noimg">▶</div>
                    {/if}

                    <div class="ep-hover-overlay">
                      <div class="cr-play-circle">
                        <svg viewBox="0 0 24 24" fill="currentColor">
                          <path d="M8 5v14l11-7z" />
                        </svg>
                      </div>
                    </div>

                    <div class="ep-duration">24m</div>

                    {#if prog && (prog.completed || percent > 0)}
                      <div class="ep-prog-track">
                        <div class="ep-prog-bar" style="width: {prog.completed ? 100 : percent}%"></div>
                      </div>
                    {/if}
                  </div>

                  <div class="ep-details">
                    <div class="ep-meta-header">
                      <span class="ep-num-tag">E{ep.number}</span>
                      <span class="ep-subdub-badge">Sub | Dub</span>
                      {#if prog?.completed}
                        <span class="ep-watched-tag">Watched</span>
                      {/if}
                    </div>

                    <h4 class="ep-title" title={ep.title}>{ep.title}</h4>

                    <p class="ep-snippet">
                      {getEpisodeSynopsis(card, ep.number)}
                    </p>
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        </section>
      {:else if activeTab === "characters"}
        <section class="characters-section">
          {#if characters.length === 0}
            <div class="empty-state">
              <p>No character information available.</p>
            </div>
          {:else}
            <div class="char-grid">
              {#each characters as ch (ch.id)}
                <div class="char-card">
                  <div class="char-img">
                    {#if ch.image}
                      <img src={ch.image} alt={ch.name} loading="lazy" />
                    {/if}
                  </div>
                  <div class="char-name" title={ch.name}>{ch.name}</div>
                  <div class="char-role">{ch.role}{ch.voiceActor ? ` · ${ch.voiceActor}` : ""}</div>
                </div>
              {/each}
            </div>
          {/if}
        </section>
      {:else if activeTab === "comments"}
        <section class="comments-section standalone">
          <h2 class="comments-heading">{commentsList.length} Comments</h2>

          <form class="comment-box" onsubmit={submitComment}>
            <div class="comment-input-row">
              <div class="user-avatar-circle">
                <img
                  src="https://images.unsplash.com/photo-1578632767115-351597cf2477?w=100&auto=format&fit=crop&q=80"
                  alt="User avatar"
                />
              </div>
              <textarea
                bind:value={newCommentText}
                placeholder="Type here to leave a comment..."
                rows="3"
              ></textarea>
            </div>

            <div class="comment-footer">
              <label class="spoiler-toggle">
                <input type="checkbox" bind:checked={isSpoiler} />
                <span>Contains spoilers</span>
              </label>

              <button type="submit" class="cr-orange-pill sm">
                Post Comment
              </button>
            </div>
          </form>

          <div class="comments-list">
            {#each commentsList as c (c.id)}
              <div class="comment-item">
                <div class="comment-avatar">
                  <img src={c.avatar} alt={c.author} />
                </div>
                <div class="comment-main">
                  <div class="comment-author-row">
                    <span class="author-name">{c.author}</span>
                    <span class="comment-time">{c.timeAgo}</span>
                  </div>

                  <div class="comment-body">
                    {c.text}
                  </div>

                  <div class="comment-actions-bar">
                    <button class="action-btn text-link">Hide replies</button>

                    <div class="vote-group">
                      <span class="vote-count">{c.votes >= 0 ? `+${c.votes}` : c.votes}</span>
                      <button
                        class="action-btn icon-only"
                        class:voted={c.userVoted === "up"}
                        onclick={() => voteComment(c, "up")}
                        title="Upvote"
                      >
                        👍
                      </button>
                      <button
                        class="action-btn icon-only"
                        class:voted={c.userVoted === "down"}
                        onclick={() => voteComment(c, "down")}
                        title="Downvote"
                      >
                        👎
                      </button>
                      <button class="action-btn icon-only flag" title="Report">
                        ⚑
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        </section>
      {/if}
    </div>
  {/if}
</div>

<style>
  .details-page {
    padding-bottom: 80px;
    background: #0d0d0d;
  }

  /* Full bleed banner hero matching Image 1 */
  .banner-hero {
    position: relative;
    height: 75vh;
    min-height: 520px;
    max-height: 680px;
    overflow: hidden;
    background: #0d0d0d;
  }

  .banner-bg {
    position: absolute;
    inset: 0;
    background-size: cover;
    background-position: center 20%;
    transform: scale(1.02);
  }

  .banner-shade {
    position: absolute;
    inset: 0;
    background:
      linear-gradient(to top, #0d0d0d 4%, rgba(13, 13, 13, 0.72) 40%, rgba(13, 13, 13, 0.25) 75%, transparent 100%),
      linear-gradient(to right, rgba(13, 13, 13, 0.94) 0%, rgba(13, 13, 13, 0.7) 45%, rgba(13, 13, 13, 0.2) 80%, transparent 100%);
  }

  .banner-content {
    position: relative;
    height: 100%;
    max-width: 1600px;
    margin: 0 auto;
    padding: calc(var(--nav-h) + 20px) 48px 40px;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    max-width: 720px;
    margin-left: 48px;
  }

  .meta-tag-row {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 12px;
  }

  .type-badge {
    font-size: 11px;
    font-weight: 800;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: #ffffff;
    background: rgba(255, 255, 255, 0.18);
    padding: 3px 8px;
    border-radius: var(--radius-sm);
  }

  .sub-badge {
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.04em;
    color: var(--text-dim);
    padding: 3px 0;
  }

  .anime-title {
    font-size: 46px;
    line-height: 1.05;
    letter-spacing: -0.025em;
    margin-bottom: 14px;
    color: #ffffff;
    font-weight: 900;
    text-transform: uppercase;
    text-shadow: 0 2px 20px rgba(0, 0, 0, 0.9);
  }

  .synopsis-hero {
    color: #d1d2d6;
    font-size: 14px;
    line-height: 1.6;
    margin: 0 0 16px;
    display: -webkit-box;
    -webkit-line-clamp: 3;
    line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
    text-shadow: 0 1px 6px rgba(0, 0, 0, 0.8);
  }

  .hero-meta-strip {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 12px;
    font-size: 13.5px;
    font-weight: 700;
    color: #c4c4ca;
    flex-wrap: wrap;
  }

  .meta-sep {
    color: #64646c;
  }

  .rating-badge {
    padding: 2px 6px;
    border-radius: 3px;
    background: rgba(255, 255, 255, 0.14);
    color: #ffffff;
    font-size: 11px;
    font-weight: 800;
  }

  .meta-subs {
    color: #a0a0a8;
    font-size: 12.5px;
  }

  .rating-strip {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 22px;
  }

  .stars {
    color: #ffffff;
    font-size: 17px;
    letter-spacing: 2px;
  }

  .votes-count {
    color: #a0a0a5;
    font-size: 13px;
    font-weight: 600;
  }

  .hero-actions {
    display: flex;
    align-items: center;
    gap: 14px;
    flex-wrap: wrap;
  }

  .cr-orange-pill {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 12px 26px;
    border-radius: 4px;
    background: var(--accent);
    color: #0d0d0d;
    font-size: 13.5px;
    font-weight: 800;
    letter-spacing: 0.04em;
    transition: background 0.14s ease, transform 0.1s ease;
  }

  .cr-orange-pill:hover {
    background: var(--accent-hover);
  }

  .cr-orange-pill.sm {
    padding: 8px 18px;
    font-size: 12.5px;
  }

  .play-icon {
    width: 17px;
    height: 17px;
    fill: #0d0d0d;
  }

  .cr-outline-pill {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 11px 22px;
    border-radius: 4px;
    background: rgba(20, 21, 25, 0.8);
    border: 1px solid rgba(255, 255, 255, 0.3);
    color: #ffffff;
    font-size: 13px;
    font-weight: 800;
    transition: all 0.14s ease;
  }

  .cr-outline-pill:hover {
    background: rgba(255, 255, 255, 0.12);
    border-color: #ffffff;
  }

  /* Season Nav Wrap */
  .season-nav-wrap {
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    background: #0d0d0d;
    padding: 0 48px;
  }

  .season-nav {
    max-width: 1600px;
    margin: 0 auto;
    display: flex;
    gap: 32px;
  }

  .season-tab {
    padding: 16px 4px;
    font-size: 15px;
    font-weight: 800;
    color: #929298;
    border-bottom: 3px solid transparent;
    margin-bottom: -1px;
    transition: color 0.14s ease, border-color 0.14s ease;
    cursor: pointer;
    background: transparent;
    border-top: none;
    border-left: none;
    border-right: none;
    font-family: inherit;
  }

  .season-tab:hover {
    color: #ffffff;
  }

  .season-tab.active {
    color: var(--accent);
    border-bottom-color: var(--accent);
  }

  /* Details body */
  .details-body {
    max-width: 1600px;
    margin: 0 auto;
    padding: 32px 48px;
  }

  /* Episodes section */
  .episodes-section {
    margin-bottom: 50px;
  }

  .section-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 20px;
    gap: 16px;
    flex-wrap: wrap;
  }

  .heading-left {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .ep-section-heading {
    font-size: 20px;
    font-weight: 800;
    color: #ffffff;
    letter-spacing: -0.02em;
    margin: 0;
  }

  .ep-count-pill {
    font-size: 12px;
    font-weight: 700;
    color: #9c9ca3;
    background: rgba(255, 255, 255, 0.08);
    padding: 2px 8px;
    border-radius: 999px;
  }

  .ep-toolbar-right {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .ep-tool-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 5px 10px;
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 4px;
    color: #cfcfd4;
    font-size: 12px;
    font-weight: 700;
    cursor: pointer;
    transition: background 0.12s ease, color 0.12s ease;
  }

  .ep-tool-btn:hover {
    background: rgba(255, 255, 255, 0.12);
    color: #ffffff;
  }

  .view-toggle-wrap {
    display: flex;
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 4px;
    padding: 2px;
  }

  .view-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 26px;
    border-radius: 3px;
    background: transparent;
    border: none;
    color: #8c8c92;
    cursor: pointer;
    transition: background 0.12s ease, color 0.12s ease;
  }

  .view-btn:hover {
    color: #ffffff;
  }

  .view-btn.active {
    background: rgba(255, 255, 255, 0.15);
    color: #ffffff;
  }

  .carousel-arrows {
    display: flex;
    gap: 6px;
  }

  .arrow-btn {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.12);
    color: #ffffff;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 18px;
    line-height: 1;
    cursor: pointer;
    transition: background 0.14s ease;
  }

  .arrow-btn:hover {
    background: rgba(255, 255, 255, 0.16);
    color: var(--accent);
  }

  /* Compact Episode Grid */
  .episodes-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 20px 16px;
  }

  /* Compact Episode Carousel */
  .episodes-carousel {
    display: flex;
    gap: 18px;
    overflow-x: auto;
    scroll-behavior: smooth;
    padding-bottom: 12px;
    scrollbar-width: thin;
  }

  .carousel-card {
    width: 255px;
    min-width: 255px;
    flex-shrink: 0;
  }

  .ep-card {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .ep-thumb {
    position: relative;
    aspect-ratio: 16 / 9;
    border-radius: 4px;
    overflow: hidden;
    background: #191a1f;
    cursor: pointer;
    border: 1px solid rgba(255, 255, 255, 0.06);
    transition: border-color 0.14s ease, transform 0.14s ease;
  }

  .ep-thumb:hover {
    border-color: var(--accent);
  }

  .ep-thumb img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
  }

  .ep-noimg {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--accent);
    font-size: 24px;
  }

  .ep-hover-overlay {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(0, 0, 0, 0.4);
    opacity: 0;
    transition: opacity 0.15s ease;
  }

  .ep-thumb:hover .ep-hover-overlay {
    opacity: 1;
  }

  .cr-play-circle {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background: var(--accent);
    color: var(--accent-contrast, #000000);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .cr-play-circle svg {
    width: 18px;
    height: 18px;
    margin-left: 2px;
  }

  .ep-duration {
    position: absolute;
    bottom: 7px;
    right: 7px;
    background: rgba(0, 0, 0, 0.85);
    color: #ffffff;
    font-size: 11px;
    font-weight: 700;
    padding: 1px 5px;
    border-radius: 3px;
    letter-spacing: 0.02em;
    pointer-events: none;
  }

  /* Progress Bar: Clean, pinned to bottom edge, reflecting actual user watch state */
  .ep-prog-track {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 3px;
    background: rgba(255, 255, 255, 0.2);
  }

  .ep-prog-bar {
    height: 100%;
    background: var(--accent);
  }

  .ep-details {
    padding: 0 2px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .ep-meta-header {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  /* Prominent Episode Number */
  .ep-num-tag {
    font-size: 13px;
    font-weight: 900;
    color: var(--accent);
    letter-spacing: 0.02em;
  }

  .ep-subdub-badge {
    font-size: 11px;
    font-weight: 600;
    color: #8c8c92;
  }

  .ep-watched-tag {
    font-size: 10px;
    font-weight: 800;
    letter-spacing: 0.04em;
    color: #10b981;
    background: rgba(16, 185, 129, 0.12);
    padding: 1px 5px;
    border-radius: 2px;
    margin-left: auto;
  }

  .ep-title {
    font-size: 13.5px;
    font-weight: 700;
    color: #ffffff;
    margin: 0;
    line-height: 1.35;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .ep-snippet {
    font-size: 12px;
    line-height: 1.45;
    color: #8c8c94;
    margin: 0;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  /* Comments section (Image 2) */
  .comments-section {
    max-width: 900px;
    border-top: 1px solid rgba(255, 255, 255, 0.08);
    padding-top: 36px;
  }

  .comments-section.standalone {
    border-top: none;
    padding-top: 0;
  }

  .comments-heading {
    font-size: 20px;
    font-weight: 800;
    color: #ffffff;
    margin-bottom: 20px;
  }

  .comment-box {
    background: #141519;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 16px;
    margin-bottom: 32px;
  }

  .comment-input-row {
    display: flex;
    gap: 14px;
    align-items: flex-start;
  }

  .user-avatar-circle {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    overflow: hidden;
    border: 1.5px solid var(--accent);
    flex-shrink: 0;
    background: #333;
  }

  .user-avatar-circle img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .comment-input-row textarea {
    flex: 1;
    background: transparent;
    border: none;
    color: #ffffff;
    font-size: 14px;
    line-height: 1.5;
    resize: vertical;
    padding: 4px;
  }

  .comment-input-row textarea:focus {
    background: transparent;
  }

  .comment-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-top: 12px;
    padding-top: 12px;
    border-top: 1px solid rgba(255, 255, 255, 0.06);
  }

  .spoiler-toggle {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 13px;
    color: #a0a0a5;
    cursor: pointer;
  }

  .spoiler-toggle input {
    cursor: pointer;
    accent-color: var(--accent);
  }

  .comments-list {
    display: flex;
    flex-direction: column;
    gap: 22px;
  }

  .comment-item {
    display: flex;
    gap: 14px;
    align-items: flex-start;
  }

  .comment-avatar {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    overflow: hidden;
    flex-shrink: 0;
    background: #23252b;
  }

  .comment-avatar img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .comment-main {
    flex: 1;
  }

  .comment-author-row {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 4px;
  }

  .author-name {
    font-size: 13.5px;
    font-weight: 800;
    color: #ffffff;
  }

  .comment-time {
    font-size: 12px;
    color: #727278;
  }

  .comment-body {
    font-size: 14px;
    line-height: 1.55;
    color: #d1d2d6;
    margin-bottom: 8px;
  }

  .comment-actions-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 12.5px;
    color: #a0a0a5;
  }

  .action-btn {
    cursor: pointer;
    color: inherit;
    transition: color 0.12s ease;
  }

  .action-btn:hover {
    color: #ffffff;
  }

  .text-link {
    font-weight: 700;
  }

  .vote-group {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .vote-count {
    font-family: var(--mono);
    font-weight: 800;
    color: #16c2d5;
    margin-right: 4px;
  }

  .action-btn.icon-only {
    font-size: 14px;
    padding: 2px 4px;
    border-radius: 4px;
  }

  .action-btn.icon-only.voted {
    color: var(--accent);
  }

  .action-btn.flag {
    color: #727278;
    margin-left: 6px;
  }

  /* Characters */
  .char-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: 20px;
  }

  .char-card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 10px;
  }

  .char-img {
    aspect-ratio: 3 / 4;
    border-radius: var(--radius-sm);
    overflow: hidden;
    background: var(--surface-2);
    margin-bottom: 10px;
  }

  .char-img img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .char-name {
    font-size: 13px;
    font-weight: 700;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  .char-role {
    font-family: var(--mono);
    font-size: 11px;
    color: var(--text-faint);
    margin-top: 2px;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  /* Quick Seasons Bar */
  .seasons-bar-container {
    background: #111217;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    padding: 12px 48px;
  }

  .seasons-bar-inner {
    max-width: 1600px;
    margin: 0 auto;
    display: flex;
    align-items: center;
    gap: 16px;
    flex-wrap: wrap;
  }

  .seasons-bar-title {
    font-size: 13px;
    font-weight: 800;
    color: var(--accent);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .seasons-chips-row {
    display: flex;
    gap: 10px;
    flex-wrap: wrap;
  }

  .season-chip {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 14px;
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: var(--radius-sm, 4px);
    color: #c5c6cc;
    cursor: pointer;
    font-family: inherit;
    font-size: 13px;
    font-weight: 700;
    transition: all 0.12s ease;
  }

  .season-chip:hover {
    background: rgba(255, 255, 255, 0.12);
    color: #ffffff;
    border-color: rgba(255, 255, 255, 0.25);
  }

  .season-chip.active {
    background: var(--accent);
    color: #000000;
    border-color: var(--accent);
    font-weight: 800;
  }

  .season-chip-eps {
    font-size: 11px;
    opacity: 0.85;
  }

  .ep-tool-btn.icon-only-sort {
    width: 36px;
    height: 36px;
    padding: 0;
    aspect-ratio: 1 / 1;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  /* Info tab */
  .info-layout {
    display: grid;
    grid-template-columns: 1fr 360px;
    gap: 48px;
  }

  .about-block {
    margin-bottom: 32px;
  }

  .block-header-flex {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 14px;
  }

  .view-all-link {
    background: transparent;
    border: none;
    color: var(--accent);
    font-size: 12.5px;
    font-weight: 800;
    cursor: pointer;
    padding: 0;
  }

  .view-all-link:hover {
    text-decoration: underline;
  }

  .block-title {
    font-size: 17px;
    font-weight: 800;
    letter-spacing: -0.01em;
    margin-bottom: 14px;
    color: var(--text, #ffffff);
  }

  .synopsis {
    color: var(--text-dim, #92939c);
    line-height: 1.7;
    margin: 0;
    white-space: pre-line;
    font-size: 14px;
  }

  /* Alt Titles Grid */
  .alt-titles-grid {
    display: flex;
    flex-direction: column;
    gap: 8px;
    background: var(--surface, #141519);
    border: 1px solid var(--border, #282a33);
    border-radius: var(--radius, 4px);
    padding: 14px 18px;
  }

  .alt-title-row {
    display: flex;
    gap: 12px;
    font-size: 13.5px;
  }

  .alt-label {
    width: 120px;
    color: var(--text-faint, #6e707c);
    font-weight: 700;
    flex-shrink: 0;
  }

  .alt-val {
    color: var(--text, #ffffff);
    font-weight: 600;
  }

  /* Related Seasons Grid */
  .related-seasons-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
    gap: 12px;
  }

  .season-card-rel {
    display: flex;
    align-items: center;
    gap: 14px;
    padding: 12px 16px;
    background: var(--surface, #141519);
    border: 1px solid var(--border, #282a33);
    border-radius: var(--radius, 4px);
    transition: all 0.12s ease;
  }

  .season-card-rel.current {
    border-color: var(--accent);
    background: var(--surface-2, #1a1b22);
  }

  .rel-season-num {
    font-size: 18px;
    font-weight: 900;
    color: var(--accent);
    background: var(--accent-dim);
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-sm, 4px);
    flex-shrink: 0;
  }

  .rel-season-info {
    flex: 1;
    min-width: 0;
  }

  .rel-season-name {
    font-size: 13.5px;
    font-weight: 800;
    color: var(--text, #ffffff);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .rel-season-sub {
    font-size: 11.5px;
    color: var(--text-dim, #7e808e);
  }

  /* Cast Preview */
  .cast-preview-row {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(130px, 1fr));
    gap: 12px;
  }

  .cast-preview-card {
    background: var(--surface, #141519);
    border: 1px solid var(--border, #282a33);
    border-radius: var(--radius, 4px);
    padding: 10px;
    text-align: center;
  }

  .cast-img {
    width: 100%;
    aspect-ratio: 1 / 1;
    object-fit: cover;
    border-radius: var(--radius-sm, 4px);
    margin-bottom: 8px;
  }

  .cast-placeholder {
    width: 100%;
    aspect-ratio: 1 / 1;
    background: var(--surface-2, #20222a);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 24px;
    font-weight: 800;
    color: var(--accent);
    border-radius: var(--radius-sm, 4px);
    margin-bottom: 8px;
  }

  .cast-name {
    font-size: 12.5px;
    font-weight: 800;
    color: var(--text, #ffffff);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .cast-role {
    font-size: 11px;
    color: var(--text-dim, #7e808e);
  }

  .genre-pills {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }

  .genre-pill {
    padding: 6px 16px;
    border-radius: var(--radius, 4px);
    background: var(--surface-2, #181920);
    border: 1px solid var(--border, #282a33);
    font-size: 12px;
    font-weight: 700;
    color: var(--text-dim, #c0c2cc);
    transition: background 0.13s ease, color 0.13s ease;
  }

  .genre-pill:hover {
    background: var(--surface-3, #22242e);
    color: var(--text, #ffffff);
    border-color: var(--accent);
  }

  .info-side {
    background: var(--surface, #141519);
    border: 1px solid var(--border, #282a33);
    border-radius: var(--radius, 6px);
    padding: 24px;
    height: fit-content;
  }

  .info-row {
    display: flex;
    justify-content: space-between;
    gap: 16px;
    padding: 11px 0;
    border-bottom: 1px solid var(--border-soft, #22242e);
    font-size: 13px;
  }

  .info-row:last-child {
    border-bottom: none;
  }

  .info-label {
    color: var(--text-faint, #7a7c88);
    font-weight: 700;
  }

  .info-value {
    font-weight: 700;
    color: var(--text, #ffffff);
    text-align: right;
  }
</style>
