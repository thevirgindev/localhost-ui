<script lang="ts">
  function scrollToTop() {
    window.scrollTo({ top: 0, behavior: "smooth" });
  }
</script>

<footer class="luci-footer" id="main-footer">
  <div class="footer-inner">
    <span class="copyright">© 2026 Luci</span>
    <button class="top-btn" onclick={scrollToTop} title="Back to top">
      <span>Top</span>
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="13" height="13">
        <polyline points="18 15 12 9 6 15" />
      </svg>
    </button>
  </div>
</footer>

<style>
  .luci-footer {
    width: 100%;
    border-top: 1px solid var(--border, #1f222b);
    color: var(--text-dim, #9699a6);
    margin-top: 40px;
    padding: 12px 0;
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
    color: var(--text, #ffffff);
  }
</style>
