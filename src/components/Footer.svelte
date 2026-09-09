<script lang="ts">
  function scrollToTop() {
    window.scrollTo({ top: 0, behavior: "smooth" });
  }
</script>

<footer class="luci-footer" id="main-footer">
  <div class="footer-inner">
    <div class="footer-brand">
      <span class="brand-mark">
        <svg viewBox="0 0 24 24" fill="none" width="14" height="14">
          <path
            d="M12 4.5l1.4 3.9a2 2 0 0 0 1.2 1.2l3.9 1.4-3.9 1.4a2 2 0 0 0-1.2 1.2L12 17.5l-1.4-3.9a2 2 0 0 0-1.2-1.2L5.5 11l3.9-1.4a2 2 0 0 0 1.2-1.2z"
            fill="currentColor"
          />
        </svg>
      </span>
      <span class="brand-name">Luci</span>
      <span class="status-dot"></span>
      <span class="status-text">Local Mode</span>
    </div>

    <div class="footer-right">
      <span class="copyright">© 2026 Luci</span>
      <button class="top-btn" onclick={scrollToTop} title="Back to top">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="13" height="13">
          <polyline points="18 15 12 9 6 15" />
        </svg>
        <span>Top</span>
      </button>
    </div>
  </div>
</footer>

<style>
  .luci-footer {
    width: 100%;
    border-top: 1px solid var(--border, #1f222b);
    color: var(--text-dim, #9699a6);
    margin-top: 48px;
    padding: 14px 0;
  }

  .footer-inner {
    max-width: 1600px;
    margin: 0 auto;
    padding: 0 44px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
  }

  @media (max-width: 640px) {
    .footer-inner {
      padding: 0 20px;
    }
  }

  .footer-brand {
    display: flex;
    align-items: center;
    gap: 9px;
  }

  .brand-mark {
    width: 24px;
    height: 24px;
    background: var(--accent, #a855f7);
    color: #0d0d0d;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .brand-name {
    font-size: 14px;
    font-weight: 900;
    letter-spacing: -0.02em;
    color: var(--text, #ffffff);
  }

  .status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--green, #10b981);
    margin-left: 8px;
  }

  .status-text {
    font-size: 11.5px;
    font-weight: 600;
    color: var(--text-faint, #6f7282);
  }

  .footer-right {
    display: flex;
    align-items: center;
    gap: 14px;
  }

  .copyright {
    font-size: 11.5px;
    color: var(--text-faint, #626574);
  }

  .top-btn {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    background: transparent;
    border: none;
    color: var(--text-dim, #8e91a0);
    font-size: 11.5px;
    font-weight: 700;
    cursor: pointer;
    font-family: inherit;
    transition: color 0.12s ease;
    padding: 4px 6px;
  }

  .top-btn:hover {
    color: var(--accent, #a855f7);
  }
</style>
