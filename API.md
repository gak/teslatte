# Tesla API Matrix

List of all known Tesla APIs, and if this crate supports it, and which of the Tesla enpoints support them.

### Legend

- Blank - Unknown
- ✅ Supported by this crate
- 🟢 Supported by specified API
- 🔴 Not supported by specified API

Currently only the Owner API is partially supported by this crate.





<!-- tesla_api_coverage start table -->
| API | Teslatte | Timdorr API | Fleet API | Command Mode |
| --- | --- | --- | --- | --- |
| actuate-trunk |  | 🟢 | 🟢 |  |
| add-key |  | 🟢 |  | 🟢 |
| add-key-request |  |  |  | 🟢 |
| add-managed-charging-site |  | 🟢 |  |  |
| adjust-volume |  | 🟢 | 🟢 |  |
| app-feedback-entitlements |  | 🟢 |  |  |
| app-feedback-logs |  | 🟢 |  |  |
| app-feedback-metadata |  | 🟢 |  |  |
| auto-conditioning-start | ✅ | 🟢 | 🟢 |  |
| auto-conditioning-stop | ✅ | 🟢 | 🟢 |  |
| auto-seat-and-climate |  |  |  | 🟢 |
| autosecure-modelx |  |  |  | 🟢 |
| backup-key |  |  | 🟢 |  |
| backup-reserve |  | 🟢 |  |  |
| calendar-history-data |  | 🟢 |  |  |
| calendar-sync |  | 🟢 |  |  |
| cancel-software-update |  | 🟢 | 🟢 | 🟢 |
| charge-max-range | ✅ |  | 🟢 |  |
| charge-port-close |  |  |  | 🟢 |
| charge-port-door-close | ✅ | 🟢 | 🟢 |  |
| charge-port-door-open | ✅ | 🟢 | 🟢 |  |
| charge-port-open |  |  |  | 🟢 |
| charge-standard | ✅ |  | 🟢 |  |
| charge-start | ✅ | 🟢 | 🟢 |  |
| charge-stop | ✅ | 🟢 | 🟢 |  |
| charging-sessions-only-for-business-fleet-owners |  |  | 🟢 |  |
| charging-set-limit |  |  |  | 🟢 |
| charging-start |  |  |  | 🟢 |
| charging-stop |  |  |  | 🟢 |
| check-energy-product-registration |  | 🟢 |  |  |
| climate-off |  |  |  | 🟢 |
| climate-on |  |  |  | 🟢 |
| climate-set-temp |  |  |  | 🟢 |
| create-energy-site-share-invite |  | 🟢 |  |  |
| dashcam-save-clip |  | 🟢 |  |  |
| deactivate-device-token |  | 🟢 |  |  |
| door-lock | ✅ | 🟢 | 🟢 |  |
| door-unlock | ✅ | 🟢 | 🟢 |  |
| drive |  |  |  | 🟢 |
| driving-plan |  | 🟢 |  |  |
| energy-event |  | 🟢 |  |  |
| energy-register-product |  | 🟢 |  |  |
| energy-site-backup-time-remaining |  | 🟢 |  |  |
| energy-site-command |  | 🟢 |  |  |
| energy-site-enroll-program |  | 🟢 |  |  |
| energy-site-import-export-config |  | 🟢 |  |  |
| energy-site-onboarding-tips |  | 🟢 |  |  |
| energy-site-opt-event |  | 🟢 |  |  |
| energy-site-preference |  | 🟢 |  |  |
| energy-site-program-details |  | 🟢 |  |  |
| energy-site-programs |  | 🟢 |  |  |
| energy-site-telemetry-history |  | 🟢 |  |  |
| energy-site-user-settings |  | 🟢 |  |  |
| energy-sites-calendar-history | ✅ |  |  |  |
| energy-sites-live-status | ✅ |  |  |  |
| energy-sites-site-info | ✅ |  |  |  |
| energy-sites-site-status | ✅ |  |  |  |
| energy-wall-connector-firmware-download-url |  | 🟢 |  |  |
| erase-user-data |  |  | 🟢 |  |
| feature-config |  | 🟢 | 🟢 |  |
| fetch-energy-site-share-invites |  | 🟢 |  |  |
| fetch-energy-site-shared-users |  | 🟢 |  |  |
| fetch-vehicle-shared-drivers |  | 🟢 |  |  |
| flash-lights | ✅ | 🟢 | 🟢 | 🟢 |
| frunk-open |  |  |  | 🟢 |
| get |  |  |  | 🟢 |
| get-charge-on-solar-feature |  | 🟢 |  |  |
| get-managed-charging-sites |  | 🟢 |  |  |
| get-ownership-xp-config |  | 🟢 |  |  |
| get-upcoming-service-visit-data |  | 🟢 |  |  |
| guest-mode |  |  | 🟢 |  |
| hermes-authorization |  | 🟢 |  |  |
| hermes-vehicle-authorization |  | 🟢 |  |  |
| honk |  |  |  | 🟢 |
| honk-horn | ✅ | 🟢 | 🟢 |  |
| list-keys |  |  |  | 🟢 |
| lock |  |  |  | 🟢 |
| mattermost |  | 🟢 |  |  |
| me |  | 🟢 | 🟢 |  |
| media-next-fav |  | 🟢 | 🟢 |  |
| media-next-track |  | 🟢 | 🟢 |  |
| media-prev-fav |  | 🟢 | 🟢 |  |
| media-prev-track |  | 🟢 | 🟢 |  |
| media-set-volume |  |  |  | 🟢 |
| media-toggle-playback |  | 🟢 | 🟢 |  |
| media-volume-down |  | 🟢 | 🟢 |  |
| media-volume-up |  | 🟢 |  |  |
| message-center-message |  | 🟢 |  |  |
| message-center-message-action-update |  | 🟢 |  |  |
| message-center-message-count |  | 🟢 |  |  |
| message-center-message-list |  | 🟢 |  |  |
| mobile-enabled |  |  | 🟢 |  |
| navigation-gps-request |  | 🟢 | 🟢 |  |
| navigation-request |  |  | 🟢 |  |
| navigation-route |  | 🟢 |  |  |
| navigation-sc-request |  | 🟢 | 🟢 |  |
| nearby-charging-sites |  | 🟢 | 🟢 |  |
| off-grid-vehicle-charging-reserve |  | 🟢 |  |  |
| onboarding-experience |  | 🟢 |  |  |
| onboarding-experience-page |  | 🟢 |  |  |
| operation-mode |  | 🟢 |  |  |
| orders |  | 🟢 | 🟢 |  |
| ping |  |  |  | 🟢 |
| place-suggestions |  | 🟢 |  |  |
| plan-trip |  | 🟢 |  |  |
| post |  |  |  | 🟢 |
| powerwall-energy-history | ✅ |  |  |  |
| powerwall-order-page |  | 🟢 |  |  |
| powerwall-order-session-data |  | 🟢 |  |  |
| powerwall-status | ✅ |  |  |  |
| product-info |  |  |  | 🟢 |
| products | ✅ | 🟢 |  |  |
| public-key |  |  | 🟢 |  |
| rate-tariffs |  | 🟢 |  |  |
| redeem-energy-site-share-invite |  | 🟢 |  |  |
| redeem-vehicle-share-invite |  | 🟢 |  |  |
| referral-data |  | 🟢 |  |  |
| referral-page |  | 🟢 |  |  |
| region |  |  | 🟢 |  |
| register |  |  | 🟢 |  |
| release-notes |  | 🟢 |  |  |
| remote-auto-seat-climate-request |  | 🟢 | 🟢 |  |
| remote-auto-steering-wheel-heat-climate-request |  | 🟢 | 🟢 |  |
| remote-boombox |  | 🟢 | 🟢 |  |
| remote-seat-cooler-request |  | 🟢 | 🟢 |  |
| remote-seat-heater-request |  | 🟢 | 🟢 |  |
| remote-start-drive | ✅ | 🟢 | 🟢 |  |
| remote-steering-wheel-heat-level-request |  | 🟢 | 🟢 |  |
| remote-steering-wheel-heater-request |  | 🟢 | 🟢 |  |
| remove-energy-site-share-user |  | 🟢 |  |  |
| remove-key |  |  |  | 🟢 |
| remove-managed-charging-site |  | 🟢 |  |  |
| remove-vehicle-share-driver |  | 🟢 |  |  |
| rename-key |  |  |  | 🟢 |
| reset-pin-to-drive-pin |  |  | 🟢 |  |
| reset-valet-pin |  | 🟢 | 🟢 |  |
| retrieve-notification-preferences |  | 🟢 |  |  |
| reverse-geocoding |  | 🟢 |  |  |
| revoke-energy-site-share-invite |  | 🟢 |  |  |
| revoke-vehicle-share-invite |  | 🟢 |  |  |
| roadside-assistance-data |  | 🟢 |  |  |
| roadside-assistance-page |  | 🟢 |  |  |
| schedule-software-update |  | 🟢 | 🟢 |  |
| seat-heater |  |  |  | 🟢 |
| send-device-key |  | 🟢 |  |  |
| send-notification-confirmation |  | 🟢 |  |  |
| send-notification-preferences |  | 🟢 |  |  |
| send-to-vehicle |  | 🟢 |  |  |
| sentry-mode |  |  |  | 🟢 |
| service-data |  | 🟢 | 🟢 |  |
| session-info |  |  |  | 🟢 |
| set-bioweapon-mode |  | 🟢 | 🟢 |  |
| set-cabin-overheat-protection |  | 🟢 | 🟢 |  |
| set-charge-limit | ✅ | 🟢 | 🟢 |  |
| set-charging-amps | ✅ | 🟢 | 🟢 |  |
| set-climate-keeper-mode |  | 🟢 | 🟢 |  |
| set-cop-temp |  | 🟢 | 🟢 |  |
| set-managed-charge-current-request |  |  | 🟢 |  |
| set-managed-charger-location |  |  | 🟢 |  |
| set-managed-scheduled-charging-time |  |  | 🟢 |  |
| set-pin-to-drive |  |  | 🟢 |  |
| set-preconditioning-max |  | 🟢 | 🟢 |  |
| set-scheduled-charging | ✅ | 🟢 | 🟢 |  |
| set-scheduled-departure | ✅ | 🟢 | 🟢 |  |
| set-sentry-mode |  | 🟢 | 🟢 |  |
| set-temps | ✅ | 🟢 | 🟢 |  |
| set-valet-mode |  | 🟢 | 🟢 |  |
| set-vehicle-name |  |  | 🟢 |  |
| share-invites |  | 🟢 | 🟢 |  |
| share-invites-create |  | 🟢 | 🟢 |  |
| share-invites-revoke |  |  | 🟢 |  |
| site-address |  | 🟢 |  |  |
| site-config |  | 🟢 |  |  |
| site-data |  | 🟢 |  |  |
| site-name |  | 🟢 |  |  |
| site-tariff |  | 🟢 |  |  |
| software-update-start |  |  |  | 🟢 |
| solar-savings-forecast |  | 🟢 |  |  |
| speed-limit-activate |  | 🟢 | 🟢 |  |
| speed-limit-clear-pin |  | 🟢 | 🟢 |  |
| speed-limit-deactivate |  | 🟢 | 🟢 |  |
| speed-limit-set-limit |  | 🟢 | 🟢 |  |
| splunk-telemetry |  | 🟢 |  |  |
| static-charger-file |  | 🟢 |  |  |
| static-supercharger-file |  | 🟢 |  |  |
| status |  | 🟢 |  |  |
| steering-wheel-heater |  |  |  | 🟢 |
| storm-mode-settings |  | 🟢 |  |  |
| sun-roof-control |  | 🟢 | 🟢 |  |
| take-drivenote |  | 🟢 | 🟢 |  |
| time-of-use-settings |  | 🟢 |  |  |
| trigger-homelink |  | 🟢 | 🟢 |  |
| trigger-vehicle-screenshot |  | 🟢 |  |  |
| trunk-close |  |  |  | 🟢 |
| trunk-move |  |  |  | 🟢 |
| trunk-open |  |  |  | 🟢 |
| unlock |  |  |  | 🟢 |
| update-calendar-entries |  |  | 🟢 |  |
| update-charge-on-solar-feature |  | 🟢 |  |  |
| user-account-upload-profile-picture |  | 🟢 |  |  |
| user-reset-vault |  | 🟢 |  |  |
| vehicle |  |  | 🟢 |  |
| vehicle-charge-history |  | 🟢 |  |  |
| vehicle-data | ✅ | 🟢 | 🟢 |  |
| vehicle-download-vault |  | 🟢 |  |  |
| vehicle-energy-sites |  | 🟢 |  |  |
| vehicle-subscriptions |  |  | 🟢 |  |
| vehicle-subscriptions-set |  |  | 🟢 |  |
| vehicle-summary |  | 🟢 |  |  |
| vehicle-upload-vault |  | 🟢 |  |  |
| vehicles | ✅ | 🟢 | 🟢 |  |
| wake |  |  |  | 🟢 |
| wake-up | ✅ | 🟢 | 🟢 |  |
| warranty-details |  |  | 🟢 |  |
| window-control |  | 🟢 | 🟢 |  |
<!-- tesla_api_coverage end table -->




