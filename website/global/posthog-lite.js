// PostHog Lite - Minimal analytics tracking
// This is a lightweight implementation for basic event tracking

class PostHogLite {
  constructor(apiKey, options = {}) {
    this.apiKey = apiKey;
    this.host = options.host || 'https://us.i.posthog.com';
    this.personProfiles = options.person_profiles || 'identified_only';
    this.distinctId = this.getDistinctId();
    this.sessionId = this.getSessionId();
    this.properties = {};

    // Capture initial pageview
    if (options.capturePageview !== false) {
      this.capture('$pageview', {
        $current_url: window.location.href,
        $pathname: window.location.pathname
      });
    }
  }

  getDistinctId() {
    let distinctId = localStorage.getItem('ph_distinct_id');
    if (!distinctId) {
      distinctId = 'ph_' + Math.random().toString(36).substring(2, 15) +
                    Math.random().toString(36).substring(2, 15);
      localStorage.setItem('ph_distinct_id', distinctId);
    }
    return distinctId;
  }

  getSessionId() {
    const now = Date.now();
    let session = JSON.parse(localStorage.getItem('ph_session') || '{}');

    // Session expires after 30 minutes of inactivity
    if (!session.id || now - session.lastActivity > 30 * 60 * 1000) {
      session = {
        id: 'session_' + now + '_' + Math.random().toString(36).substring(2, 9),
        lastActivity: now
      };
    } else {
      session.lastActivity = now;
    }

    localStorage.setItem('ph_session', JSON.stringify(session));
    return session.id;
  }

  capture(eventName, properties = {}) {
    const referrer = document.referrer || '$direct';
    const referringDomain = document.referrer ? new URL(document.referrer).hostname : '$direct';

    const payload = {
      api_key: this.apiKey,
      event: eventName,
      properties: {
        ...this.properties,
        ...properties,
        distinct_id: this.distinctId,
        $session_id: this.sessionId,
        $insert_id: Math.random().toString(36).substring(2, 10) + Math.random().toString(36).substring(2, 10),
        $lib: 'web',
        $lib_version: '1.0.0',
        $current_url: window.location.href,
        $host: window.location.host,
        $pathname: window.location.pathname,
        $title: document.title,
        $referrer: referrer,
        $referring_domain: referringDomain,
        $screen_height: window.screen.height,
        $screen_width: window.screen.width,
        $viewport_height: window.innerHeight,
        $viewport_width: window.innerWidth,
      },
      timestamp: new Date().toISOString()
    };

    // Send to PostHog
    this.send(payload);
  }

  send(payload) {
    const endpoint = `${this.host}/capture/`;

    // Use sendBeacon if available, fallback to fetch
    if (navigator.sendBeacon) {
      const blob = new Blob([JSON.stringify(payload)], { type: 'application/json' });
      navigator.sendBeacon(endpoint, blob);
    } else {
      fetch(endpoint, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(payload),
        keepalive: true
      }).catch(err => {
        console.error('PostHog capture failed:', err);
      });
    }
  }

  register(properties) {
    this.properties = { ...this.properties, ...properties };
  }

  identify(distinctId, properties = {}) {
    this.distinctId = distinctId;
    localStorage.setItem('ph_distinct_id', distinctId);

    if (Object.keys(properties).length > 0) {
      this.capture('$identify', {
        $set: properties,
        distinct_id: distinctId
      });
    }
  }

  reset() {
    localStorage.removeItem('ph_distinct_id');
    localStorage.removeItem('ph_session');
    this.distinctId = this.getDistinctId();
    this.sessionId = this.getSessionId();
    this.properties = {};
  }
}

// Initialize PostHog globally
if (typeof window !== 'undefined') {
  window.PostHogLite = PostHogLite;

  // Auto-initialize if API key is set
  if (typeof POSTHOG_API_KEY !== 'undefined') {
    window.posthog = new PostHogLite(POSTHOG_API_KEY, window.POSTHOG_OPTIONS || {});
  }
}
