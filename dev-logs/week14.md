# CKBuilder Track Weekly Report — Week 14
- Name: **Mayowa Temitope AKINYELE**
- Week Ending: **Apr 14, 2026**

## Artifacts Completed
All artifacts are actually local. They are my brainstorming sessions. I will be asking Phil to check the .drawio out later.

## Key Learnings
- It was about making sure I was not walking into build mode with a vague product idea and a pile of disconnected inspirations.
- The most important thing I did was narrow the field. The ecosystem scan began wide. `Fiber Checkout` emerged as the product that turns CKB's unusual strengths into something ordinary merchants can actually understand and use.
- I came out of the research more convinced that the strongest CKB opportunities are not generic copies of already-crowded categories. The stronger bet is taking CKB's cell model, account abstraction, RGB++, Fiber, and object standards, then packaging them into a business surface that feels normal from the outside.
- The scaffolding also made me separate product layers more cleanly. Research, PRD, architecture, information architecture, user flow, design system, component library, diagramming, and execution roadmap are different jobs. Mixing them too early usually produces mush.
- The merchant metaphor also became much sharper. Serious operators want sessions, links, invoices, settlements, payouts, reports, webhooks, and exports. They do not want a dashboard that teaches them channels, nodes, rebalancing, or cell anatomy before they can get paid.
- The user-flow work made the rail abstraction click. A channel is not created per payment. Channels are long-lived liquidity roads. The merchant-facing object is the checkout session, link, invoice, or POS charge.
- The design system work mattered more than I expected because it stopped the product from drifting into generic crypto admin UI. `Measured Voltage` gave the build a point of view: controlled, infrastructural, calm, and precise.
- The payment-links roadmap was useful because it exposed what was still missing. The package was strong on direction and scaffolding, but still needed object contracts, state machines, API behavior, POS specifics, and operational edge-case policy.

## Brief
- This week was the scaffolding week for what became `Fiber Checkout`.
- I started from a broader CKB opportunity scan because I did not want to jump straight into building the first idea that sounded good. I wanted to know where CKB is actually strongest today, where the ecosystem already feels crowded, and what kind of application could turn the chain's weird advantages into something with real product gravity.
- From there I narrowed into a merchant-infrastructure direction and built a full planning pack around it inside `ckb-brainstorming/` (a local file yet). The important result is not just that I picked a direction. It is that I translated that direction into a connected set of documents that define what the product is, who it serves, how it behaves, how it should look, and what remains before implementation becomes honest.
- By the end of the week, the folder was no longer a random research dump. It had become a real scaffolding package: thesis, opportunity framing, PRD, architecture, navigation model, user flow, design system, component rules, execution roadmap, and preview assets.

## Practical Progress
- Built the ecosystem scan as the top-of-funnel document. This established the strategic context: CKB is strong in infrastructure and primitives, but thinner in polished operator-facing businesses. That gave the rest of the work a sharper standard.
- Used that scan to rank several opportunity directions, then converged on `Fiber Checkout` as the strongest wedge. The direction is simple enough to explain, but native enough to CKB and Fiber to avoid feeling like a generic clone.
- Wrote the merchant UI landscape to keep the product grounded in how serious merchant tools already behave. Stripe, Adyen, and Coinbase Commerce were treated less as style references and more as pattern libraries for trust, operational hierarchy, and link-first growth.
- Wrote the Fiber Checkout PRD to define the actual product thesis:
  - who the primary users are
  - what jobs the product is solving
  - what surfaces matter
  - what belongs in MVP
  - what explicitly does not belong in MVP
  - what the phased roadmap should look like
- Wrote the system overview to separate merchant truth from rail truth. This document locked in the main architectural stance: hide chain complexity behind stable business objects, keep platform-managed liquidity as the default, and let the ledger drive merchant-visible balances instead of raw chain observation.
- Wrote the information architecture to define where everything lives once the merchant is inside the product. This turned the product from a concept into an interface system with navigation, page purpose, content hierarchy, and command-model behavior.
- Wrote the user flow to connect merchant setup, hosted checkout, Fiber invoice creation, payment routing, settlement, and webhook delivery into one story. This is the document that makes the platform responsibilities legible without leaking raw infrastructure into the merchant experience.
- Added the draw.io user-flow diagram so the flow is not trapped in prose alone. This matters because payments architecture becomes hard to reason about once every interaction only exists in paragraphs.
- Built the design system so the product would not default into generic B2B or generic crypto styling. The work covered:
  - aesthetic direction
  - brand character
  - typography
  - color system
  - layout rules
  - motion rules
  - iconography
  - data visualization principles
- Extended that into the component library so the design system had operational parts instead of only taste language. This gave the product a usable first component grammar: app shell, metric cards, data tables, timelines, settlement ladders, payload inspectors, drawers, command bars, and empty states.
- Added design tokens and a static preview page so the system moved one step closer to implementation. The design no longer lives only as a concept note; it already has an initial token layer and a rendered preview surface.
- Wrote the payment-links roadmap as the execution honesty document. This was important because the rest of the package could have created the illusion that the product was already fully specified. The roadmap makes the missing work explicit:
  - object models
  - state machines
  - API contracts
  - webhook semantics
  - POS behavior
  - settlement and payout edge cases
  - rollout and observability policy

## Challenges
- The biggest challenge now is moving from strong scaffolding into a disciplined first implementation. The package is coherent, but implementation can still go wrong if I start coding without locking down the object model and state machines first. With some questions on the community needs, etc and use cases. 
- The payment-links surface still needs the hardest product decisions. "Payment link" sounds simple until I need to define templates, checkout sessions, invoices, POS charges, payment attempts, settlements, and webhook events precisely.
- There is also an ambition risk. Because the product direction is broad, it would be easy to overbuild treasury, FX, or enterprise controls too early and lose the narrow wedge that makes the idea credible.
- The UX challenge is subtle but serious: I need the product to feel like serious merchant infrastructure while still being distinct. That is why the design system matters so much. If the implementation loses that restraint, the product will start to look like either another crypto dashboard or another generic admin template.
- Finally, the CKB/Fiber fit still needs to be proven in execution, not just argued in planning. The scaffolding says the product should hide rail complexity behind stable business objects. The real test will be whether the implementation can keep that promise under live payment, settlement, and failure conditions.
