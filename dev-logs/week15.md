# CKBuilder Track Weekly Report — Week 15
- Name: **Mayowa Temitope AKINYELE**
- Week Ending: **Apr 21, 2026**

## Artifacts Completed
All of the main artifacts for this week are local inside `ckb-brainstorming/`. The important one is the execution-grade payment links roadmap that pushed `Fiber Checkout` beyond broad scaffolding and into product-definition work that can actually guide implementation honestly.

## Courses / Readings Completed
- [Use a prebuilt Stripe-hosted payment page](https://docs.stripe.com/payments/checkout)
- [How Checkout works](https://docs.stripe.com/payments/checkout/how-checkout-works)
- [Stripe Payment Links](https://docs.stripe.com/payments/payment-links)
- [Stripe Payment Link API Reference](https://docs.stripe.com/api/payment-link)
- [Track a payment link](https://docs.stripe.com/payment-links/url-parameters)
- [Customize Checkout](https://docs.stripe.com/payments/checkout/customization)
- [Customize the appearance of Checkout](https://docs.stripe.com/payments/checkout/customization/appearance)
- [Branding your Stripe account](https://docs.stripe.com/get-started/account/branding)
- [Handle payment events with webhooks](https://docs.stripe.com/webhooks/handling-payment-events)
- [Adyen Online Payments](https://docs.adyen.com/online-payments)
- [Adyen Checkout API Overview](https://docs.adyen.com/api-explorer/Checkout/latest/overview)
- [Adyen Webhooks Overview](https://docs.adyen.com/api-explorer/Webhooks/latest/overview)
- [Adyen Create a payment link](https://docs.adyen.com/api-explorer/Checkout/72/post/paymentLinks)
- [Adyen Payment links through the Customer Area](https://docs.adyen.com/unified-commerce/pay-by-link/create-payment-links/customer-area)
- [Adyen Webhook best practices](https://docs.adyen.com/development-resources/webhooks/best-practices)
- [Coinbase Payment Link API Reference](https://docs.cdp.coinbase.com/api-reference/business-api/rest-api/payment-links/introduction)
- [Coinbase Payment Link Webhooks](https://docs.cdp.coinbase.com/coinbase-business/payment-link-apis/webhooks)
- [Connect Public Nodes on Fiber](https://www.fiber.world/docs/quick-start/connect-nodes)
- [The Case For RGB++](https://www.nervos.org/knowledge-base/the-case-for-rgbpp)

## Key Learnings
- This week made me much less comfortable with vague words like "payment link." The phrase sounds simple, but it can hide very different product objects, lifecycles, and user expectations.
- I came away convinced that `Fiber Checkout` needs multiple merchant-facing payment request primitives, not one overloaded object. Hosted checkout sessions, reusable payment links, invoices, and POS charges solve related problems, but they are not the same surface.
- The Stripe docs were especially useful because they make the product boundary explicit. A payment link is reusable, but each visit still resolves into a session-level payment flow. That reinforced my decision not to collapse templates and live payment state into one object.
- The Stripe customization and branding material also reminded me that trust is visual and operational at the same time. Checkout pages are not just forms. They are trust surfaces where naming consistency, recognizable branding, domain control, and restraint reduce confusion and chargeback risk.
- Adyen sharpened the operational side of my thinking. Their docs keep returning to the same architecture pattern: payments server, client surface, and webhook server. That gave me a cleaner mental model for how `Fiber Checkout` should separate orchestration, buyer experience, and post-payment state changes.
- The Adyen payment-link docs also helped me think more clearly about idempotency, status transitions, forced expiry, and payment-method control. Those details are easy to postpone in a brainstorm, but they are exactly what separates a merchant tool from a pretty demo.
- The Coinbase payment-link material was useful as a crypto-native comparison point. It reminded me that crypto checkout products often converge on the same merchant-facing primitives even when the rail and settlement logic underneath are very different.
- Another useful takeaway from the webhook reading is that post-payment truth rarely arrives only inside the synchronous checkout request. Real merchant systems need to survive duplicate events, retries, out-of-band updates, and delayed settlement signals. That made the webhook contract feel less like an accessory and more like part of the core product.
- The object model matters more than interface polish at this stage. If I do not define `checkout_session`, `payment_link`, `invoice`, `pos_charge`, `payment_attempt`, `settlement_entry`, and `webhook_event` clearly, the product will look coherent in diagrams but behave ambiguously in real use.
- State machines are part of product design, not only backend design. Expiration, underpayment, overpayment, duplicate attempts, delayed settlement, webhook failure, payout holds, and refunds all shape what merchants trust.
- The POS angle became sharper too. `BlackBox` is an exciting use case, but it should not distort the whole foundation. The right move is proving payment links and hosted checkout first, then layering POS behavior on top of stable primitives.
- I also understood more clearly that merchant-safe accounting cannot just mirror raw rail events. The platform needs canonical payment and settlement events that protect the merchant from Fiber and chain-level complexity leaking upward.
- The Fiber and RGB++ reading helped me keep the CKB angle honest. The product cannot just borrow the surface language of Stripe and ignore the reasons CKB/Fiber are interesting. The merchant abstraction has to stay clean, but the settlement thesis still depends on Fiber channels, Bitcoin-adjacent flows, and RGB++-style asset logic being genuinely useful underneath.

## Brief
- This week was about forcing `Fiber Checkout` to become more precise.
- The earlier work already gave me a strong direction: merchant infrastructure over CKB and Fiber, with a Stripe-like control plane and a calmer, more serious design system. But that package still had a risk. It could feel complete while still hiding the hardest product questions.
- A large part of the progress this week was not code or even architecture prose in the narrow sense. It was comparative reading. I spent more time looking at how mature payment products explain themselves, structure their payment request surfaces, handle trust, and think about merchant operations. That reading changed the shape of the roadmap more than another speculative feature list would have.
- So I used this week to narrow the next serious implementation wedge: payment links, hosted checkout, invoices, and POS charges. The goal was not to add more surface area for its own sake. The goal was to define what is actually missing before implementation starts pretending to know more than it does.
- By the end of the week, the work had shifted from broad product scaffolding into execution planning. The system now has a clearer sense of what belongs in the first build, what objects it depends on, what API and webhook contracts are missing, and where operational edge cases will become real.

## Practical Progress
- Consolidated a much broader product research base around mainstream and crypto-native payment products so the roadmap would not be built from intuition alone.
- Wrote the `payment-links-roadmap.md` document as the main execution bridge between the existing planning package and the actual product build.
- Defined the recommended product taxonomy for `Fiber Checkout`:
  - hosted checkout session
  - payment link
  - invoice
  - POS charge
- Locked in the recommendation that a `payment_link` should behave as a reusable template that generates one or more `checkout_session` objects. This keeps analytics, retries, and settlement cleaner than treating the link itself as the whole payment state.
- Identified the highest-priority missing artifact as the canonical object model for:
  - `checkout_session`
  - `payment_link`
  - `invoice`
  - `pos_charge`
  - `payment_attempt`
  - `settlement_entry`
  - `webhook_event`
- Defined the main link modes that make sense to support first:
  - fixed amount
  - customer-entered amount
  - fixed inventory item
  - invoice-derived
  - POS quick charge
- Explicitly ruled out early launch complexity that would bloat the wedge too soon:
  - subscriptions
  - split payments
  - marketplace routing
  - offline settlement promises
- Surfaced the need for separate state machines for checkout sessions, payment attempts, settlement entries, and payouts so the product can behave predictably under stress.
- Mapped the next API and webhook definition layer, including:
  - OpenAPI schema work
  - example request and response payloads
  - idempotency behavior
  - test/live key behavior
  - webhook signature rules
  - event naming conventions
- Used Stripe, Adyen, and Coinbase as pattern references not just for visuals but for product boundaries:
  - hosted page versus embedded flow
  - reusable link versus one-time session
  - webhook-first settlement updates
  - low-code merchant paths versus API-first flows
- Pulled more trust-surface lessons from mainstream payment products:
  - consistent business naming
  - controlled branding zones rather than infinite customization
  - explicit expiration and status treatment
  - domain and checkout-surface consistency
  - clear handling of merchant-facing payment-method choices
- Clarified the sequencing around `BlackBox` POS. The recommendation is to validate payment links and hosted checkout first, prove the accounting and settlement model there, and only then build the POS layer on top of those proven primitives.
- Consolidated the local package with a clearer `README.md` so the research, PRD, architecture, information architecture, user flow, roadmap, design system, and diagramming all read as one connected body of work instead of a loose folder of notes.

## Challenges
- The biggest challenge is still false completeness. The documents look strong, but a strong package can create the illusion that implementation is ready before the object contracts and state transitions are actually locked in.
- Payment links remain deceptively difficult. As soon as I move from the phrase into the real product, I have to define templates, sessions, invoices, attempts, settlements, retries, expirations, and payouts much more carefully than a merchant-facing UI might suggest.
- There is also a sequencing risk with POS. It is tempting to overfit early thinking to the most vivid device use case, but that could warp the core model before the more general payment surfaces are stable.
- The final challenge is keeping the product merchant-native while the rails remain CKB- and Fiber-specific underneath. The system only works if that abstraction remains disciplined under live settlement and failure conditions.
