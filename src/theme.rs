use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// 主题作用域基础样式与组件样式片段。
macro_rules! adui_scope_style {
    () => {
        r#"
.adui-theme-scope {
    color: var(--adui-color-text);
    background-color: var(--adui-color-bg-base);
    font-family: "Segoe UI", "SF Pro Text", system-ui, -apple-system, sans-serif;
    line-height: var(--adui-line-height, 1.5715);
}
"#
    };
}

macro_rules! adui_calendar_style {
    () => {
        r#"
.adui-calendar {
    display: inline-block;
    padding: 8px 12px;
    border-radius: var(--adui-radius, 4px);
    border: 1px solid var(--adui-color-border, #d9d9d9);
    background-color: var(--adui-color-bg-container, #ffffff);
}

.adui-calendar-fullscreen {
    width: 100%;
}

.adui-calendar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 8px;
}

.adui-calendar-nav-btn {
    border: none;
    background: transparent;
    cursor: pointer;
    padding: 0 4px;
}

.adui-calendar-header-view {
    font-weight: 500;
}

.adui-calendar-week-row {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 2px;
    margin-bottom: 4px;
    font-size: 12px;
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
}

.adui-calendar-week-cell {
    text-align: center;
}

.adui-calendar-body {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 2px;
}

.adui-calendar-date {
    box-sizing: border-box;
}

.adui-calendar-date-empty {
    cursor: default;
}

.adui-calendar-date-cell {
    padding: 4px 0;
    border-radius: var(--adui-radius-sm, 4px);
    text-align: center;
    cursor: pointer;
}

.adui-calendar-date-cell:hover {
    background-color: var(--adui-color-bg-base, #f5f5f5);
}

.adui-calendar-date-selected .adui-calendar-date-cell {
    background-color: var(--adui-color-primary, #1677ff);
    color: #fff;
}

.adui-calendar-date-value {
    display: inline-block;
}
"#
    };
}

macro_rules! adui_time_picker_style {
    () => {
        r#"
.adui-time-picker-root {
    position: relative;
    display: inline-block;
}

.adui-time-picker {
    display: inline-flex;
    align-items: center;
    box-sizing: border-box;
    min-width: 160px;
    width: auto;
    min-height: var(--adui-control-height, 32px);
    padding: 4px 8px;
    border-radius: var(--adui-radius, 4px);
    border: 1px solid var(--adui-color-border, #d9d9d9);
    background-color: var(--adui-color-bg-container, #ffffff);
    cursor: text;
}

.adui-time-picker-disabled {
    background-color: var(--adui-color-bg-base, #f5f5f5);
    cursor: not-allowed;
    opacity: 0.6;
}

.adui-time-picker-input {
    flex: 1;
    border: none;
    outline: none;
    background: transparent;
    padding: 0;
    margin: 0;
    font: inherit;
}

.adui-time-picker-clear {
    margin-left: 4px;
    cursor: pointer;
    user-select: none;
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
}

.adui-time-picker-dropdown {
    margin-top: 4px;
    padding: 8px 12px;
    border-radius: var(--adui-radius, 4px);
    background-color: var(--adui-color-bg-container, #ffffff);
    box-shadow: var(--adui-shadow, 0 3px 6px rgba(0,0,0,0.12));
}

.adui-time-picker-panel {
    display: flex;
    gap: 8px;
}

.adui-time-picker-column {
    max-height: 200px;
    overflow-y: auto;
}

.adui-time-picker-cell {
    display: block;
    min-width: 40px;
    padding: 4px 0;
    text-align: center;
    border-radius: var(--adui-radius-sm, 4px);
    cursor: pointer;
}

.adui-time-picker-cell:hover {
    background-color: var(--adui-color-bg-base, #f5f5f5);
}

.adui-time-picker-cell-active {
    background-color: var(--adui-color-primary, #1677ff);
    color: #fff;
}
"#
    };
}

macro_rules! adui_button_style {
    () => {
        r#"
.adui-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    border: var(--adui-control-line-width, 1px) solid var(--adui-btn-border);
    border-style: var(--adui-btn-border-style, solid);
    background: var(--adui-btn-bg);
    color: var(--adui-btn-color);
    padding: var(--adui-btn-padding-block) var(--adui-btn-padding-inline);
    height: var(--adui-btn-height);
    min-width: 0;
    border-radius: var(--adui-btn-radius);
    font-size: var(--adui-btn-font-size);
    line-height: 1.2;
    cursor: pointer;
    transition: all var(--adui-motion-duration-fast, 0.18s) ease;
    box-shadow: var(--adui-btn-shadow, none);
    text-decoration: none;
    user-select: none;
}

.adui-btn:hover:not(.adui-btn-disabled) {
    background: var(--adui-btn-bg-hover);
    color: var(--adui-btn-color-hover);
    border-color: var(--adui-btn-border-hover);
}

.adui-btn:active:not(.adui-btn-disabled) {
    background: var(--adui-btn-bg-active);
    color: var(--adui-btn-color-active);
    border-color: var(--adui-btn-border-active);
    box-shadow: none;
}

.adui-btn:focus-visible {
    outline: none;
    box-shadow: var(--adui-btn-focus-shadow, 0 0 0 2px rgba(22, 119, 255, 0.3));
}

.adui-btn-dashed {
    border-style: dashed;
}

.adui-btn-text,
.adui-btn-link {
    border-color: transparent;
    background: transparent;
}

.adui-btn-link {
    box-shadow: none;
}

.adui-btn-primary {
    color: #fff;
}

.adui-btn-disabled {
    cursor: not-allowed;
    opacity: 0.65;
}

.adui-btn-block {
    width: 100%;
}

.adui-btn-ghost {
    background: transparent;
}

.adui-btn-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
}

.adui-btn-spinner {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    border: 2px solid currentColor;
    border-right-color: transparent;
    animation: adui-spin 0.9s linear infinite;
}

.adui-btn-group {
    display: inline-flex;
    align-items: stretch;
}

.adui-btn-group > .adui-btn {
    border-radius: 0;
}

.adui-btn-group > .adui-btn:first-child {
    border-top-left-radius: var(--adui-btn-radius);
    border-bottom-left-radius: var(--adui-btn-radius);
}

.adui-btn-group > .adui-btn:last-child {
    border-top-right-radius: var(--adui-btn-radius);
    border-bottom-right-radius: var(--adui-btn-radius);
}

.adui-btn-group > .adui-btn:not(:last-child) {
    margin-right: -1px;
}

[dir="rtl"] .adui-btn-group {
    direction: rtl;
}

[dir="rtl"] .adui-btn-group > .adui-btn:first-child {
    border-top-right-radius: var(--adui-btn-radius);
    border-bottom-right-radius: var(--adui-btn-radius);
    border-top-left-radius: 0;
    border-bottom-left-radius: 0;
}

[dir="rtl"] .adui-btn-group > .adui-btn:last-child {
    border-top-left-radius: var(--adui-btn-radius);
    border-bottom-left-radius: var(--adui-btn-radius);
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
}

[dir="rtl"] .adui-btn-group > .adui-btn:not(:last-child) {
    margin-right: 0;
    margin-left: -1px;
}
"#
    };
}

macro_rules! adui_icon_style {
    () => {
        r#"
.adui-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    vertical-align: middle;
    transition: transform 0.16s ease;
}

.adui-icon-spin {
    animation: adui-spin 1s linear infinite;
}
"#
    };
}

macro_rules! adui_divider_style {
    () => {
        r#"
.adui-divider {
    border-top: 1px solid var(--adui-color-split, var(--adui-color-border));
    margin: 16px 0;
    position: relative;
}

.adui-divider-vertical {
    display: inline-block;
    height: 1em;
    border-left: 1px solid var(--adui-color-split, var(--adui-color-border));
    margin: 0 8px;
}

.adui-divider-horizontal {
    display: flex;
    align-items: center;
}

.adui-divider-plain {
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
    font-size: 13px;
}

.adui-divider-dashed {
    border-top-style: dashed;
}

.adui-divider-inner-text {
    padding: 0 8px;
    background: transparent;
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
    font-size: 13px;
}

.adui-divider-left::before,
.adui-divider-right::after,
.adui-divider-center::before,
.adui-divider-center::after {
    content: "";
    border-top: 1px solid var(--adui-color-split, var(--adui-color-border));
    flex: 1;
}

.adui-divider-left { justify-content: flex-start; gap: 8px; }
.adui-divider-center { justify-content: center; gap: 8px; }
.adui-divider-right { justify-content: flex-end; gap: 8px; }
"#
    };
}

macro_rules! adui_alert_style {
    () => {
        r#"
.adui-alert {
    position: relative;
    display: flex;
    align-items: flex-start;
    padding: 8px 12px;
    border-radius: var(--adui-radius, 4px);
    border: 1px solid var(--adui-color-border);
    background-color: var(--adui-color-bg-container);
    color: var(--adui-color-text);
    font-size: 13px;
    line-height: 1.4;
}

.adui-alert + .adui-alert {
    margin-top: 8px;
}

.adui-alert-icon {
    margin-right: 8px;
    display: flex;
    align-items: center;
    padding-top: 1px;
}

.adui-alert-content {
    flex: 1;
}

.adui-alert-message {
    font-weight: 500;
}

.adui-alert-description {
    margin-top: 4px;
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
}

.adui-alert-close-icon {
    margin-left: 8px;
    border: none;
    background: transparent;
    cursor: pointer;
    color: inherit;
    padding: 0;
}

.adui-alert-success {
    border-color: var(--adui-color-success-border, #b7eb8f);
    background-color: var(--adui-color-success-bg, #f6ffed);
    color: var(--adui-color-success-text, #3f6600);
}

.adui-alert-info {
    border-color: var(--adui-color-info-border, #91d5ff);
    background-color: var(--adui-color-info-bg, #e6f7ff);
    color: var(--adui-color-info-text, #09599b);
}

.adui-alert-warning {
    border-color: var(--adui-color-warning-border, #ffe58f);
    background-color: var(--adui-color-warning-bg, #fffbe6);
    color: var(--adui-color-warning-text, #613400);
}

.adui-alert-error {
    border-color: var(--adui-color-error-border, #ffa39e);
    background-color: var(--adui-color-error-bg, #fff1f0);
    color: var(--adui-color-error-text, #a8071a);
}

.adui-alert-banner {
    border-radius: 0;
    border-left: none;
    border-right: none;
}
"#
    };
}

macro_rules! adui_date_picker_style {
    () => {
        r#"
.adui-date-picker-root {
    position: relative;
    display: inline-block;
}

.adui-date-picker,
.adui-date-picker-range {
    display: inline-flex;
    align-items: center;
    box-sizing: border-box;
    min-width: 220px;
    width: auto;
    min-height: var(--adui-control-height, 32px);
    padding: 4px 8px;
    border-radius: var(--adui-radius, 4px);
    border: 1px solid var(--adui-color-border, #d9d9d9);
    background-color: var(--adui-color-bg-container, #ffffff);
    cursor: text;
}

.adui-date-picker-disabled {
    background-color: var(--adui-color-bg-base, #f5f5f5);
    cursor: not-allowed;
    opacity: 0.6;
}

.adui-date-picker-input {
    flex: 1;
    border: none;
    outline: none;
    background: transparent;
    padding: 0;
    margin: 0;
    font: inherit;
}

.adui-date-picker-input-start,
.adui-date-picker-input-end {
    flex: 1;
}

.adui-date-picker-range-separator {
    padding: 0 4px;
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
}

.adui-date-picker-clear {
    margin-left: 4px;
    cursor: pointer;
    user-select: none;
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
}

.adui-date-picker-dropdown {
    margin-top: 4px;
    padding: 8px 12px;
    border-radius: var(--adui-radius, 4px);
    background-color: var(--adui-color-bg-container, #ffffff);
    box-shadow: var(--adui-shadow, 0 3px 6px rgba(0,0,0,0.12));
}

.adui-date-picker-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 8px;
}

.adui-date-picker-nav-btn {
    border: none;
    background: transparent;
    cursor: pointer;
    padding: 0 4px;
}

.adui-date-picker-header-view {
    font-weight: 500;
}

.adui-date-picker-week-row {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 2px;
    margin-bottom: 4px;
    font-size: 12px;
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
}

.adui-date-picker-week-cell {
    text-align: center;
}

.adui-date-picker-body {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 2px;
}

.adui-date-picker-cell {
    box-sizing: border-box;
    text-align: center;
    padding: 4px 0;
    border-radius: var(--adui-radius-sm, 4px);
    cursor: pointer;
}

.adui-date-picker-cell-empty {
    cursor: default;
}

.adui-date-picker-cell-date:hover {
    background-color: var(--adui-color-bg-base, #f5f5f5);
}

.adui-date-picker-cell-selected {
    background-color: var(--adui-color-primary, #1677ff);
    color: #fff;
}

.adui-date-picker-cell-in-range {
    background-color: rgba(22, 119, 255, 0.15);
}

.adui-date-picker-cell-range-start,
.adui-date-picker-cell-range-end {
    background-color: var(--adui-color-primary, #1677ff);
    color: #fff;
}
"#
    };
}

macro_rules! adui_result_style {
    () => {
        r#"
.adui-result {
    padding: 24px 32px;
    text-align: center;
}

.adui-result-icon {
    margin-bottom: 16px;
    font-size: 0;
}

.adui-result-title {
    margin-bottom: 8px;
    font-size: 20px;
    font-weight: 600;
    color: var(--adui-color-text);
}

.adui-result-subtitle {
    margin-bottom: 16px;
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
}

.adui-result-extra {
    margin-top: 16px;
}

.adui-result-content {
    margin-top: 24px;
    text-align: left;
}

.adui-result-success .adui-result-icon {
    color: var(--adui-color-success, #52c41a);
}

.adui-result-info .adui-result-icon {
    color: var(--adui-color-primary, #1677ff);
}

.adui-result-warning .adui-result-icon {
    color: var(--adui-color-warning, #faad14);
}

.adui-result-error .adui-result-icon,
.adui-result-403 .adui-result-icon,
.adui-result-404 .adui-result-icon,
.adui-result-500 .adui-result-icon {
    color: var(--adui-color-error, #ff4d4f);
}
"#
    };
}

macro_rules! adui_steps_style {
    () => {
        r#"
.adui-steps {
    display: flex;
    list-style: none;
    padding: 0;
    margin: 0;
}

.adui-steps-horizontal {
    flex-direction: row;
    gap: 16px;
}

.adui-steps-vertical {
    flex-direction: column;
    gap: 12px;
}

.adui-steps-item {
    display: flex;
    align-items: flex-start;
}

.adui-steps-item-icon {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    border: 1px solid var(--adui-color-border);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    margin-right: 8px;
    background: var(--adui-color-bg-container);
    font-size: 13px;
}

.adui-steps-item-index {
    line-height: 1;
}

.adui-steps-item-content {
    display: flex;
    flex-direction: column;
}

.adui-steps-item-title {
    font-size: var(--adui-font-size, 14px);
    color: var(--adui-color-text);
}

.adui-steps-item-description {
    margin-top: 2px;
    font-size: var(--adui-font-size-sm, 13px);
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
}

.adui-steps-status-wait .adui-steps-item-icon {
    border-color: var(--adui-color-border);
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
}

.adui-steps-status-process .adui-steps-item-icon {
    border-color: var(--adui-color-primary, #1677ff);
    background: var(--adui-color-primary, #1677ff);
    color: #fff;
}

.adui-steps-status-finish .adui-steps-item-icon {
    border-color: var(--adui-color-success, #52c41a);
    background: var(--adui-color-success, #52c41a);
    color: #fff;
}

.adui-steps-status-error .adui-steps-item-icon {
    border-color: var(--adui-color-error, #ff4d4f);
    background: var(--adui-color-error, #ff4d4f);
    color: #fff;
}

.adui-steps-item-disabled {
    cursor: not-allowed;
    opacity: 0.6;
}

.adui-steps-sm .adui-steps-item-title {
    font-size: var(--adui-font-size-sm, 13px);
}

.adui-steps-lg .adui-steps-item-title {
    font-size: var(--adui-font-size-lg, 16px);
}
"#
    };
}

macro_rules! adui_typography_style {
    () => {
        r#"
.adui-text {
    font-size: inherit;
    line-height: var(--adui-line-height, 1.5715);
    word-break: break-word;
}

.adui-text-code {
    padding: 1px 4px;
    background: rgba(0, 0, 0, 0.04);
    border: 1px solid rgba(0, 0, 0, 0.06);
    border-radius: 6px;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
    font-size: 0.95em;
}

.adui-text-mark {
    background: rgba(255, 229, 143, 0.8);
    padding: 1px 2px;
}

.adui-text-strong {
    font-weight: 600;
}

.adui-text-italic {
    font-style: italic;
}

.adui-text-nowrap {
    white-space: nowrap;
}

.adui-text-ellipsis {
    display: inline-block;
    max-width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    vertical-align: bottom;
}

.adui-text-ellipsis-multiline {
    display: -webkit-box;
    -webkit-box-orient: vertical;
    overflow: hidden;
}

.adui-text-disabled {
    color: var(--adui-color-text-disabled, rgba(0,0,0,0.25));
    cursor: not-allowed;
}

.adui-paragraph {
    margin: 0 0 0.6em 0;
}

.adui-title {
    margin: 0 0 0.4em 0;
    font-weight: 600;
    color: var(--adui-color-text);
    line-height: 1.25;
}

.adui-title-1 { font-size: 32px; }
.adui-title-2 { font-size: 28px; }
.adui-title-3 { font-size: 24px; }
.adui-title-4 { font-size: 20px; }
.adui-title-5 { font-size: 16px; }

.adui-typography-control {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    margin-left: 6px;
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
    cursor: pointer;
    border: none;
    background: transparent;
    padding: 0;
    font-size: 0.9em;
}

.adui-typography-control:hover {
    color: var(--adui-color-text, inherit);
}

.adui-typography-control[aria-disabled="true"] {
    cursor: not-allowed;
    opacity: 0.5;
}

.adui-text-editing {
    display: inline-flex;
    align-items: center;
    gap: 6px;
}

.adui-typography-input,
.adui-typography-textarea {
    border: 1px solid var(--adui-color-border);
    border-radius: var(--adui-radius-sm, 4px);
    padding: 2px 6px;
    font: inherit;
    background: var(--adui-color-bg-container);
    color: var(--adui-color-text);
    min-width: 160px;
}

.adui-typography-textarea {
    min-height: 80px;
    resize: vertical;
}

.adui-typography-edit-btn {
    border: 1px solid var(--adui-color-border);
    background: var(--adui-color-bg-container);
    border-radius: var(--adui-radius-sm, 4px);
    padding: 2px 6px;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    justify-content: center;
}

.adui-typography-edit-btn:hover {
    border-color: var(--adui-color-border-hover);
}
"#
    };
}

macro_rules! adui_layout_style {
    () => {
        r#"
.adui-layout {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
}

.adui-layout-has-sider {
    flex-direction: row;
    min-height: auto;
}

.adui-layout-header,
.adui-layout-footer {
    padding: 12px 16px;
}

.adui-layout-content {
    padding: 16px;
    flex: 1;
}

.adui-layout-sider {
    min-height: 100%;
    padding: 12px;
    display: flex;
    flex-direction: column;
    position: relative;
    transition: width var(--adui-motion-duration-mid, 0.24s) ease;
}

.adui-layout-sider-children {
    flex: 1;
}

.adui-layout-sider-trigger {
    text-align: center;
    line-height: 40px;
    height: 40px;
    cursor: pointer;
    border-top: 1px solid rgba(255, 255, 255, 0.06);
}

.adui-layout-sider-zero-width {
    padding-inline: 0;
}

.adui-layout-sider-zero-trigger {
    position: absolute;
    top: 16px;
    width: 32px;
    height: 32px;
    border-radius: var(--adui-radius);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    box-shadow: var(--adui-shadow);
    background: var(--adui-color-bg-container);
    color: var(--adui-color-text);
}

	.adui-layout-sider-zero-trigger-left {
	    right: -16px;
	}

		.adui-layout-sider-zero-trigger-right {
	    left: -16px;
	}

	/* Menu */
	.adui-menu {
	    margin: 0;
	    padding: 0;
	    list-style: none;
	    box-sizing: border-box;
	    color: var(--adui-color-text);
	    font-size: var(--adui-font-size, 14px);
	}

	.adui-menu-inline {
	    width: 100%;
	}

	.adui-menu-horizontal {
	    display: flex;
	    align-items: center;
	    gap: 8px;
	}

	.adui-menu-list {
	    list-style: none;
	    margin: 0;
	    padding: 0;
	}

	.adui-menu-item {
	    display: flex;
	    align-items: center;
	    padding: 6px 12px;
	    cursor: pointer;
	    border-radius: var(--adui-radius-sm, 4px);
	    transition: background 0.16s ease, color 0.16s ease;
	}

	.adui-menu-horizontal .adui-menu-item {
	    border-radius: 0;
	    padding: 8px 12px;
	}

	.adui-menu-item:hover:not(.adui-menu-item-disabled) {
	    background: rgba(0, 0, 0, 0.04);
	}

	.adui-menu-item-selected {
	    background: rgba(22, 119, 255, 0.08);
	    color: var(--adui-color-primary);
	}

	.adui-menu-item-disabled {
	    cursor: not-allowed;
	    color: var(--adui-color-text-disabled, rgba(0,0,0,0.25));
	}

	.adui-menu-item-title {
	    display: inline-flex;
	    align-items: center;
	    gap: 8px;
	    width: 100%;
	}

	.adui-menu-item-icon {
	    display: inline-flex;
	    align-items: center;
	    justify-content: center;
	}

	.adui-menu-item-label {
	    flex: 1;
	    min-width: 0;
	    white-space: nowrap;
	    text-overflow: ellipsis;
	    overflow: hidden;
	}

	.adui-menu-submenu-list {
	    list-style: none;
	    margin: 4px 0 0 12px;
	    padding: 0;
	}

	.adui-menu-submenu-item {
	    padding-inline-start: 12px;
	}

		.adui-menu-inline-collapsed .adui-menu-submenu-list {
		    display: none;
		}

		/* Breadcrumb */
		.adui-breadcrumb {
		    font-size: 13px;
		    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
		}

		.adui-breadcrumb-list {
		    list-style: none;
		    margin: 0;
		    padding: 0;
		    display: flex;
		    flex-wrap: wrap;
		    align-items: center;
		}

		.adui-breadcrumb-item {
		    display: inline-flex;
		    align-items: center;
		}

		.adui-breadcrumb-link {
		    color: var(--adui-color-link);
		    text-decoration: none;
		}

		.adui-breadcrumb-link:hover {
		    text-decoration: underline;
		}

		.adui-breadcrumb-text {
		    color: inherit;
		}

		.adui-breadcrumb-text-current {
		    font-weight: 500;
		    color: var(--adui-color-text, #000);
		}

		.adui-breadcrumb-separator {
		    margin: 0 4px;
		    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
		}

		/* Pagination */
		.adui-pagination {
		    display: inline-flex;
		    align-items: center;
		    gap: 8px;
		    font-size: 14px;
		    color: var(--adui-color-text);
		}

		.adui-pagination-total {
		    margin-right: 8px;
		    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
		}

		.adui-pagination-list {
		    display: inline-flex;
		    list-style: none;
		    margin: 0;
		    padding: 0;
		    gap: 4px;
		}

		.adui-pagination-item {
		    min-width: 28px;
		    height: 28px;
		    padding: 0 8px;
		    display: inline-flex;
		    align-items: center;
		    justify-content: center;
		    border-radius: var(--adui-radius-sm, 4px);
		    border: 1px solid transparent;
		    cursor: pointer;
		    transition: all var(--adui-motion-duration-fast, 0.16s) ease;
		}

		.adui-pagination-item:hover:not(.adui-pagination-item-disabled) {
		    border-color: var(--adui-color-border-hover);
		    background: rgba(0, 0, 0, 0.02);
		}

		.adui-pagination-item-active {
		    border-color: var(--adui-color-primary);
		    color: var(--adui-color-primary);
		    background: rgba(22, 119, 255, 0.08);
		}

		.adui-pagination-item-disabled {
		    cursor: not-allowed;
		    color: var(--adui-color-text-disabled, rgba(0,0,0,0.25));
		}

		.adui-pagination-item-ellipsis {
		    cursor: default;
		}

		.adui-pagination-size-changer {
		    margin-left: 4px;
		    padding: 2px 6px;
		    border-radius: var(--adui-radius-sm, 4px);
		    border: 1px solid var(--adui-color-border);
		    background: var(--adui-color-bg-container);
		    font-size: 13px;
		}
	"#
    };
}

macro_rules! adui_grid_style {
    () => {
        r#"
.adui-row {
    width: 100%;
    --adui-row-gutter-x: 0px;
    --adui-row-gutter-y: 0px;
}

.adui-col {
    box-sizing: border-box;
    padding-left: calc(var(--adui-row-gutter-x, 0px) / 2);
    padding-right: calc(var(--adui-row-gutter-x, 0px) / 2);
    padding-bottom: var(--adui-row-gutter-y, 0px);
}
"#
    };
}

macro_rules! adui_space_style {
    () => {
        r#"
.adui-space {
    display: inline-flex;
    flex-direction: row;
    flex-wrap: nowrap;
    align-items: flex-start;
    gap: 0;
}

.adui-space-horizontal { flex-direction: row; }
.adui-space-vertical { flex-direction: column; }
.adui-space-wrap { flex-wrap: wrap; }

.adui-space-align-start { align-items: flex-start; }
.adui-space-align-end { align-items: flex-end; }
.adui-space-align-center { align-items: center; }
.adui-space-align-baseline { align-items: baseline; }

.adui-space-size-small { gap: var(--adui-padding-inline-sm, 8px); }
.adui-space-size-middle { gap: var(--adui-padding-inline, 16px); }
.adui-space-size-large { gap: var(--adui-padding-inline-lg, 24px); }

.adui-space-compact { gap: 0; }
"#
    };
}

macro_rules! adui_flex_style {
    () => {
        r#"
.adui-flex {
    display: flex;
    gap: 0;
}

.adui-flex-horizontal { flex-direction: row; }
.adui-flex-horizontal.adui-flex-row-reverse { flex-direction: row-reverse; }
.adui-flex-vertical { flex-direction: column; }
.adui-flex-vertical.adui-flex-column-reverse { flex-direction: column-reverse; }

.adui-flex-wrap-nowrap { flex-wrap: nowrap; }
.adui-flex-wrap-wrap { flex-wrap: wrap; }
.adui-flex-wrap-wrap-reverse { flex-wrap: wrap-reverse; }

.adui-flex-justify-start { justify-content: flex-start; }
.adui-flex-justify-end { justify-content: flex-end; }
.adui-flex-justify-center { justify-content: center; }
.adui-flex-justify-between { justify-content: space-between; }
.adui-flex-justify-around { justify-content: space-around; }
.adui-flex-justify-evenly { justify-content: space-evenly; }

.adui-flex-align-start { align-items: flex-start; }
.adui-flex-align-end { align-items: flex-end; }
.adui-flex-align-center { align-items: center; }
.adui-flex-align-stretch { align-items: stretch; }
.adui-flex-align-baseline { align-items: baseline; }

.adui-flex-gap-small { gap: var(--adui-padding-inline-sm, 8px); }
.adui-flex-gap-middle { gap: var(--adui-padding-inline, 16px); }
.adui-flex-gap-large { gap: var(--adui-padding-inline-lg, 20px); }
"#
    };
}

macro_rules! adui_form_style {
    () => {
        r#"
.adui-form {
    width: 100%;
    display: block;
}

.adui-form-inline {
    display: inline-flex;
    flex-wrap: wrap;
    gap: 16px;
    align-items: flex-end;
}

.adui-form-item {
    display: flex;
    flex-direction: column;
    gap: 4px;
    margin-bottom: 16px;
    font-size: 14px;
}

.adui-form-horizontal .adui-form-item {
    flex-direction: row;
    align-items: flex-start;
}

.adui-form-horizontal .adui-form-item-label {
    width: 120px;
    padding-inline-end: 12px;
    text-align: right;
}

.adui-form-vertical .adui-form-item-label,
.adui-form-inline .adui-form-item-label {
    text-align: left;
    padding-inline-end: 0;
    margin-bottom: 4px;
}

.adui-form-item-label {
    display: inline-flex;
    align-items: center;
    justify-content: flex-start;
    min-height: 32px;
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
    font-size: 14px;
}

.adui-form-item-required {
    color: var(--adui-color-error);
    margin-inline-end: 4px;
}

.adui-form-item-control {
    flex: 1;
    min-width: 0;
}

.adui-form-item-control > *:first-child {
    width: 100%;
}

.adui-form-item-help {
    font-size: 13px;
    color: var(--adui-color-error);
    margin-top: 2px;
}

.adui-form-item-extra {
    font-size: 12px;
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
}

.adui-form-item-has-error .adui-form-item-control > *:first-child {
    border-color: var(--adui-color-error);
    box-shadow: 0 0 0 2px rgba(255, 77, 79, 0.1);
}

.adui-form-item-has-feedback .adui-form-item-label {
    position: relative;
}

.adui-form-small .adui-form-item-label {
    min-height: 24px;
}

.adui-form-large .adui-form-item-label {
    min-height: 40px;
}
"#
    };
}

macro_rules! adui_control_style {
    () => {
        r#"
/* Text input controls */
.adui-input {
    box-sizing: border-box;
    width: 100%;
    padding: 4px 11px;
    border-radius: var(--adui-radius-sm, 4px);
    border: 1px solid var(--adui-color-border);
    background: var(--adui-color-bg-container);
    color: var(--adui-color-text);
    font-size: var(--adui-font-size, 14px);
    line-height: var(--adui-line-height, 1.5715);
    outline: none;
    transition: all var(--adui-motion-duration-fast, 0.16s) ease;
}

.adui-input::placeholder {
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
}

.adui-input:hover {
    border-color: var(--adui-color-border-hover);
}

.adui-input:focus,
.adui-input:focus-visible {
    border-color: var(--adui-color-primary);
    box-shadow: 0 0 0 2px rgba(22, 119, 255, 0.12);
}

.adui-input[disabled] {
    background: rgba(0, 0, 0, 0.02);
    color: var(--adui-color-text-disabled, rgba(0,0,0,0.25));
    cursor: not-allowed;
}

.adui-input-textarea {
    min-height: 80px;
    resize: vertical;
}

.adui-input-affix-wrapper {
    display: inline-flex;
    align-items: center;
    box-sizing: border-box;
    width: 100%;
    padding: 0 11px;
    border-radius: var(--adui-radius-sm, 4px);
    border: 1px solid var(--adui-color-border);
    background: var(--adui-color-bg-container);
    color: var(--adui-color-text);
    font-size: var(--adui-font-size, 14px);
    line-height: var(--adui-line-height, 1.5715);
    transition: all var(--adui-motion-duration-fast, 0.16s) ease;
}

.adui-input-affix-wrapper:hover {
    border-color: var(--adui-color-border-hover);
}

.adui-input-affix-wrapper:focus-within {
    border-color: var(--adui-color-primary);
    box-shadow: 0 0 0 2px rgba(22, 119, 255, 0.12);
}

.adui-input-prefix,
.adui-input-suffix {
    display: inline-flex;
    align-items: center;
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
}

.adui-input-prefix {
    margin-right: 4px;
}

.adui-input-suffix {
    margin-left: 4px;
}

.adui-input-clear {
    margin-left: 4px;
    cursor: pointer;
}

.adui-input-clear:hover {
    color: var(--adui-color-text, inherit);
}

/* InputNumber */
.adui-input-number {
    display: inline-flex;
    align-items: center;
    box-sizing: border-box;
    min-height: var(--adui-control-height, 32px);
    padding: 0 8px;
    border-radius: var(--adui-radius-sm, 4px);
    border: 1px solid var(--adui-color-border);
    background: var(--adui-color-bg-container);
    color: var(--adui-color-text);
    font-size: var(--adui-font-size, 14px);
    line-height: var(--adui-line-height, 1.5715);
    transition: all var(--adui-motion-duration-fast, 0.16s) ease;
}

.adui-input-number:focus-within {
    border-color: var(--adui-color-primary);
    box-shadow: 0 0 0 2px rgba(22, 119, 255, 0.12);
}

.adui-input-number-disabled {
    background: rgba(0, 0, 0, 0.02);
    color: var(--adui-color-text-disabled, rgba(0,0,0,0.25));
    cursor: not-allowed;
    opacity: 0.7;
}

.adui-input-number-input {
    flex: 1;
    border: none;
    outline: none;
    background: transparent;
    padding: 4px 0;
    color: inherit;
    font: inherit;
}

.adui-input-number-input:disabled {
    background: transparent;
}

.adui-input-number-prefix,
.adui-input-number-suffix {
    display: inline-flex;
    align-items: center;
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
}

.adui-input-number-prefix { margin-right: 4px; }
.adui-input-number-suffix { margin-left: 4px; }

.adui-input-number-handlers {
    display: flex;
    flex-direction: column;
    margin-left: 4px;
    gap: 2px;
}

.adui-input-number-handler {
    width: 18px;
    height: 12px;
    border: none;
    padding: 0;
    margin: 0;
    background: transparent;
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
    cursor: pointer;
    line-height: 1;
}

.adui-input-number-handler:hover {
    color: var(--adui-color-text, inherit);
}

.adui-input-number-handler:disabled {
    cursor: not-allowed;
    opacity: 0.4;
}

/* Slider */
.adui-slider {
    position: relative;
    width: 100%;
    height: 32px;
    padding: 12px 0;
    box-sizing: border-box;
}

.adui-slider-rail {
    position: absolute;
    left: 0;
    right: 0;
    top: 50%;
    transform: translateY(-50%);
    height: 4px;
    background: var(--adui-color-fill-secondary, rgba(0, 0, 0, 0.06));
    border-radius: 2px;
}

.adui-slider-track {
    position: absolute;
    height: 4px;
    top: 50%;
    transform: translateY(-50%);
    background: var(--adui-color-primary, #1677ff);
    border-radius: 2px;
}

.adui-slider-handle {
    position: absolute;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    border: 2px solid var(--adui-color-bg-container, #fff);
    background: var(--adui-color-primary, #1677ff);
    box-shadow: 0 0 0 2px rgba(22, 119, 255, 0.2);
    cursor: pointer;
    transform: translate(-50%, -50%);
    outline: none;
}

.adui-slider-handle:focus-visible,
.adui-slider-handle:hover {
    box-shadow: 0 0 0 4px rgba(22, 119, 255, 0.25);
}

.adui-slider-disabled .adui-slider-handle {
    background: var(--adui-color-fill-secondary, rgba(0, 0, 0, 0.06));
    cursor: not-allowed;
    box-shadow: none;
}

.adui-slider-disabled .adui-slider-track {
    background: var(--adui-color-fill-secondary, rgba(0, 0, 0, 0.06));
}

.adui-slider-marks {
    position: absolute;
    left: 0;
    right: 0;
    top: 18px;
    font-size: 12px;
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
}

.adui-slider-mark {
    position: absolute;
    text-align: center;
    transform: translateX(-50%);
}

.adui-slider-dot {
    display: inline-block;
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--adui-color-fill-secondary, rgba(0, 0, 0, 0.15));
    margin-bottom: 4px;
}

.adui-slider-vertical {
    width: 32px;
    height: 160px;
    padding: 0 12px;
}

.adui-slider-vertical .adui-slider-rail {
    width: 4px;
    height: 100%;
    left: 50%;
    right: auto;
    top: 0;
    transform: translateX(-50%);
}

.adui-slider-vertical .adui-slider-track {
    width: 4px;
    left: 50%;
    right: auto;
    transform: translateX(-50%);
    top: auto;
}

.adui-slider-vertical .adui-slider-handle {
    left: 50%;
    transform: translate(-50%, 50%);
}

.adui-slider-vertical .adui-slider-marks {
    left: 18px;
    right: auto;
    top: 0;
    bottom: 0;
}

.adui-slider-vertical .adui-slider-mark {
    transform: translateY(50%);
}

/* Rate */
.adui-rate {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    color: var(--adui-color-text, #faad14);
}

.adui-rate-star {
    position: relative;
    font-size: 20px;
    color: var(--adui-color-fill-secondary, rgba(0, 0, 0, 0.25));
    cursor: pointer;
    line-height: 1;
}

.adui-rate-star:hover {
    transform: scale(1.05);
}

.adui-rate-disabled .adui-rate-star {
    cursor: not-allowed;
    opacity: 0.6;
}

.adui-rate-star-default {
    display: inline-block;
}

.adui-rate-star-full {
    color: var(--adui-color-warning, #faad14);
}

.adui-rate-star-half {
    color: var(--adui-color-warning-hover, #ffc53d);
}

/* Segmented */
.adui-segmented {
    display: inline-flex;
    gap: 4px;
    padding: 2px;
    border-radius: var(--adui-radius, 6px);
    background: var(--adui-color-bg-base, #f5f5f5);
}

.adui-segmented-block {
    width: 100%;
}

.adui-segmented-round {
    border-radius: var(--adui-radius-lg, 999px);
}

.adui-segmented-disabled {
    opacity: 0.6;
    cursor: not-allowed;
}

.adui-segmented-item {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    min-height: 28px;
    padding: 4px 10px;
    border-radius: var(--adui-radius-sm, 6px);
    border: 1px solid transparent;
    background: transparent;
    color: var(--adui-color-text, inherit);
    cursor: pointer;
    transition: all var(--adui-motion-duration-fast, 0.16s) ease;
}

.adui-segmented-item:hover:not(:disabled) {
    background: var(--adui-color-bg-container, #ffffff);
    border-color: var(--adui-color-border, #d9d9d9);
}

.adui-segmented-item:disabled {
    cursor: not-allowed;
    color: var(--adui-color-text-disabled, rgba(0,0,0,0.25));
}

.adui-segmented-item-active {
    background: var(--adui-color-bg-container, #ffffff);
    border-color: var(--adui-color-primary, #1677ff);
    color: var(--adui-color-primary, #1677ff);
    box-shadow: 0 0 0 2px rgba(22, 119, 255, 0.12);
}

.adui-segmented-item-icon {
    display: inline-flex;
    align-items: center;
}

.adui-segmented-item-label {
    white-space: nowrap;
}

/* Color Picker */
.adui-color-picker {
    display: inline-flex;
    gap: 12px;
    align-items: flex-start;
    padding: 8px;
    border: 1px solid var(--adui-color-border, #d9d9d9);
    border-radius: var(--adui-radius, 8px);
    background: var(--adui-color-bg-container, #ffffff);
}

.adui-color-picker-disabled {
    opacity: 0.6;
    cursor: not-allowed;
}

.adui-color-picker-preview {
    width: 40px;
    height: 40px;
    border-radius: var(--adui-radius-sm, 6px);
    border: 1px solid var(--adui-color-border, #d9d9d9);
    background: transparent;
}

.adui-color-picker-controls {
    display: flex;
    flex-direction: column;
    gap: 8px;
    min-width: 180px;
}

.adui-color-picker-sat {
    position: relative;
    width: 100%;
    aspect-ratio: 1 / 1;
    border-radius: var(--adui-radius-sm, 6px);
    overflow: hidden;
    cursor: crosshair;
}

.adui-color-picker-sat-white {
    position: absolute;
    inset: 0;
    background: linear-gradient(90deg, #fff, rgba(255,255,255,0));
}

.adui-color-picker-sat-black {
    position: absolute;
    inset: 0;
    background: linear-gradient(0deg, #000, rgba(0,0,0,0));
}

.adui-color-picker-sat-handle {
    position: absolute;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    border: 2px solid #fff;
    box-shadow: 0 0 0 1px rgba(0,0,0,0.3);
    transform: translate(-50%, -50%);
    pointer-events: none;
}

.adui-color-picker-slider {
    position: relative;
    height: 12px;
    border-radius: 6px;
    border: 1px solid var(--adui-color-border, #d9d9d9);
    cursor: pointer;
}

.adui-color-picker-input-row {
    display: flex;
    gap: 8px;
    align-items: center;
}

.adui-color-picker-input {
    flex: 1;
    padding: 4px 8px;
    border: 1px solid var(--adui-color-border, #d9d9d9);
    border-radius: var(--adui-radius-sm, 4px);
    font: inherit;
}

.adui-color-picker-clear {
    padding: 4px 8px;
    border: 1px solid var(--adui-color-border, #d9d9d9);
    background: var(--adui-color-bg-base, #f5f5f5);
    border-radius: var(--adui-radius-sm, 4px);
    cursor: pointer;
}

/* Checkbox */
.adui-checkbox {
    display: inline-flex;
    align-items: center;
    cursor: pointer;
    font-size: var(--adui-font-size, 14px);
    color: var(--adui-color-text);
    gap: 8px;
}

.adui-checkbox-input {
    position: absolute;
    opacity: 0;
}

.adui-checkbox-inner {
    position: relative;
    width: 16px;
    height: 16px;
    border-radius: 4px;
    border: 1px solid var(--adui-color-border);
    background: var(--adui-color-bg-container);
    box-sizing: border-box;
    transition: all var(--adui-motion-duration-fast, 0.16s) ease;
}

.adui-checkbox-checked .adui-checkbox-inner {
    background: var(--adui-color-primary);
    border-color: var(--adui-color-primary);
}

.adui-checkbox-checked .adui-checkbox-inner::after {
    content: "";
    position: absolute;
    inset: 3px 2px;
    border: 2px solid #fff;
    border-top: 0;
    border-right: 0;
    transform: rotate(-45deg);
}

.adui-checkbox-indeterminate .adui-checkbox-inner::after {
    content: "";
    position: absolute;
    inset: 50% 3px auto 3px;
    height: 0;
    border-bottom: 2px solid #fff;
    transform: translateY(-50%);
}

.adui-checkbox-disabled {
    cursor: not-allowed;
    color: var(--adui-color-text-disabled, rgba(0,0,0,0.25));
}

.adui-checkbox-disabled .adui-checkbox-inner {
    background: rgba(0, 0, 0, 0.02);
    border-color: var(--adui-color-border);
}

.adui-checkbox-group {
    display: inline-flex;
    flex-wrap: wrap;
    gap: 8px 16px;
}

/* Radio */
.adui-radio {
    display: inline-flex;
    align-items: center;
    cursor: pointer;
    font-size: var(--adui-font-size, 14px);
    color: var(--adui-color-text);
    gap: 8px;
}

.adui-radio-input {
    position: absolute;
    opacity: 0;
}

.adui-radio-inner {
    position: relative;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    border: 1px solid var(--adui-color-border);
    background: var(--adui-color-bg-container);
    box-sizing: border-box;
    transition: all var(--adui-motion-duration-fast, 0.16s) ease;
}

.adui-radio-checked .adui-radio-inner {
    border-color: var(--adui-color-primary);
}

.adui-radio-checked .adui-radio-inner::after {
    content: "";
    position: absolute;
    inset: 4px;
    border-radius: 50%;
    background: var(--adui-color-primary);
}

.adui-radio-disabled {
    cursor: not-allowed;
    color: var(--adui-color-text-disabled, rgba(0,0,0,0.25));
}

.adui-radio-disabled .adui-radio-inner {
    background: rgba(0, 0, 0, 0.02);
    border-color: var(--adui-color-border);
}

.adui-radio-group {
    display: inline-flex;
    flex-wrap: wrap;
    gap: 8px 16px;
}

/* Switch */
.adui-switch {
    position: relative;
    display: inline-flex;
    align-items: center;
    box-sizing: border-box;
    min-width: 44px;
    height: 22px;
    border-radius: 100px;
    background: rgba(0, 0, 0, 0.25);
    cursor: pointer;
    border: none;
    padding: 2px;
    transition: all var(--adui-motion-duration-fast, 0.16s) ease;
}

.adui-switch-handle {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    background: #fff;
    box-shadow: 0 2px 4px rgba(0,0,0,0.15);
    transition: all var(--adui-motion-duration-fast, 0.16s) ease;
}

.adui-switch-checked {
    background: var(--adui-color-primary);
}

.adui-switch-checked .adui-switch-handle {
    left: calc(100% - 20px);
}

.adui-switch-inner {
    width: 100%;
    text-align: center;
    font-size: 12px;
    color: #fff;
    pointer-events: none;
}

.adui-switch-small {
    min-width: 32px;
    height: 18px;
}

.adui-switch-small .adui-switch-handle {
    width: 14px;
    height: 14px;
    top: 2px;
    left: 2px;
}

.adui-switch-small.adui-switch-checked .adui-switch-handle {
    left: calc(100% - 16px);
}

.adui-switch-disabled {
    cursor: not-allowed;
    opacity: 0.5;
}

/* Select */
.adui-select-root {
    display: inline-block;
    min-width: 120px;
}

.adui-select {
    display: inline-flex;
    align-items: center;
    box-sizing: border-box;
    width: 100%;
    min-height: 32px;
    padding: 0 11px;
    border-radius: var(--adui-radius-sm, 4px);
    border: 1px solid var(--adui-color-border);
    background: var(--adui-color-bg-container);
    color: var(--adui-color-text);
    font-size: var(--adui-font-size, 14px);
    line-height: var(--adui-line-height, 1.5715);
    transition: all var(--adui-motion-duration-fast, 0.16s) ease;
    cursor: pointer;
}

.adui-select:hover:not(.adui-select-disabled) {
    border-color: var(--adui-color-border-hover);
}

.adui-select:focus-visible,
.adui-select:focus-within {
    outline: none;
    border-color: var(--adui-color-primary);
    box-shadow: 0 0 0 2px rgba(22, 119, 255, 0.12);
}

.adui-select-disabled {
    cursor: not-allowed;
    color: var(--adui-color-text-disabled, rgba(0,0,0,0.25));
    background: rgba(0, 0, 0, 0.02);
}

.adui-select-sm {
    min-height: 24px;
    padding: 0 8px;
    font-size: 13px;
}

.adui-select-lg {
    min-height: 40px;
    padding: 0 12px;
    font-size: 15px;
}

.adui-select-selector {
    flex: 1;
    min-width: 0;
    display: inline-flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 4px;
}

.adui-select-selection-placeholder {
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
}

.adui-select-selection-item {
    max-width: 100%;
    display: inline-flex;
    align-items: center;
    padding: 0 4px;
    border-radius: 2px;
    background: transparent;
    color: inherit;
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
}

.adui-select-multiple .adui-select-selection-item {
    background: rgba(0, 0, 0, 0.04);
}

.adui-select-clear {
    margin-left: 4px;
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
    font-size: 12px;
    cursor: pointer;
}

.adui-select-clear:hover {
    color: var(--adui-color-text, inherit);
}

.adui-select-dropdown {
    margin-top: 4px;
    border-radius: var(--adui-radius-sm, 4px);
    border: 1px solid var(--adui-color-border);
    background: var(--adui-color-bg-container);
    box-shadow: 0 6px 16px rgba(0,0,0,0.08);
    max-height: 240px;
    overflow-y: auto;
    box-sizing: border-box;
}

.adui-select-search {
    padding: 4px 8px;
    border-bottom: 1px solid var(--adui-color-border);
}

.adui-select-search-input {
    width: 100%;
    box-sizing: border-box;
    padding: 4px 8px;
    border-radius: var(--adui-radius-sm, 4px);
    border: 1px solid var(--adui-color-border);
    background: var(--adui-color-bg-container);
    font-size: var(--adui-font-size, 14px);
}

.adui-select-item-list {
    list-style: none;
    margin: 0;
    padding: 4px 0;
}

.adui-select-item {
    padding: 4px 12px;
    cursor: pointer;
    font-size: var(--adui-font-size, 14px);
    color: var(--adui-color-text);
    display: flex;
    align-items: center;
    transition: background 0.12s ease;
}

.adui-select-item-option-active:not(.adui-select-item-option-disabled) {
    background: rgba(0, 0, 0, 0.04);
}

.adui-select-item-option-selected:not(.adui-select-item-option-disabled) {
    background: rgba(22, 119, 255, 0.08);
    color: var(--adui-color-primary);
}

.adui-select-item-option-disabled {
    cursor: not-allowed;
    color: var(--adui-color-text-disabled, rgba(0,0,0,0.25));
}

.adui-dropdown-root {
    position: relative;
    display: inline-block;
}

.adui-dropdown-menu {
    border-radius: var(--adui-radius-sm, 4px);
    border: 1px solid var(--adui-color-border);
    background: var(--adui-color-bg-container);
    box-shadow: 0 6px 16px rgba(0,0,0,0.08);
    box-sizing: border-box;
}

.adui-dropdown-menu-list {
    list-style: none;
    margin: 0;
    padding: 4px 0;
}

.adui-dropdown-menu-item {
    padding: 4px 12px;
    cursor: pointer;
    font-size: var(--adui-font-size, 14px);
    color: var(--adui-color-text);
    display: flex;
    align-items: center;
    transition: background 0.12s ease;
}

.adui-dropdown-menu-item:hover:not(.adui-dropdown-menu-item-disabled) {
    background: rgba(0, 0, 0, 0.04);
}

.adui-dropdown-menu-item-disabled {
    cursor: not-allowed;
    color: var(--adui-color-text-disabled, rgba(0,0,0,0.25));
}

.adui-dropdown-sm .adui-dropdown-menu-item {
    font-size: 13px;
    padding: 4px 10px;
}

.adui-dropdown-lg .adui-dropdown-menu-item {
    font-size: 15px;
    padding: 6px 14px;
}

 	/* Generic status helpers (can be applied to wrappers) */
 	.adui-control-status-success,
.adui-control-status-success:focus,
.adui-control-status-success:focus-within {
    border-color: var(--adui-color-success);
}

.adui-control-status-warning,
.adui-control-status-warning:focus,
.adui-control-status-warning:focus-within {
    border-color: var(--adui-color-warning);
}

.adui-control-status-error,
.adui-control-status-error:focus,
.adui-control-status-error:focus-within {
    border-color: var(--adui-color-error);
}
"#
    };
}

macro_rules! adui_empty_style {
    () => {
        r#"
.adui-empty {
    margin: 0;
    padding: 16px 8px;
    text-align: center;
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
    font-size: var(--adui-font-size-sm, 13px);
}

.adui-empty-sm {
    padding: 8px 4px;
}

.adui-empty-image {
    margin-bottom: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
}

.adui-empty-image-svg {
    width: 64px;
    height: 41px;
}

.adui-empty-image-simple {
    width: 40px;
    height: 32px;
    border-radius: var(--adui-radius-sm, 4px);
    background: var(--adui-color-bg-container);
    border: 1px dashed var(--adui-color-border);
}

.adui-empty-image-img {
    max-width: 100%;
    max-height: 80px;
    object-fit: contain;
}

.adui-empty-description {
    margin: 0;
    margin-top: 4px;
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
}

.adui-empty-footer {
    margin-top: 12px;
}
"#
    };
}

macro_rules! adui_spin_style {
    () => {
        r#"
.adui-spin {
    display: inline-flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 8px;
    color: var(--adui-color-primary);
}

.adui-spin-sm {
    font-size: var(--adui-font-size-sm, 13px);
}

.adui-spin-lg {
    font-size: var(--adui-font-size-lg, 16px);
}

.adui-spin-indicator {
    display: inline-flex;
    align-items: center;
    justify-content: center;
}

.adui-spin-dot {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    border: 2px solid currentColor;
    border-right-color: transparent;
    animation: adui-spin 0.9s linear infinite;
}

.adui-spin-text {
    font-size: var(--adui-font-size-sm, 13px);
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
}

.adui-spin-nested {
    position: relative;
    display: block;
}

.adui-spin-nested-container {
    position: relative;
}

.adui-spin-nested-mask {
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background: rgba(255, 255, 255, 0.65);
}

.adui-theme-scope[theme-mode="dark"] .adui-spin-nested-mask {
    background: rgba(0, 0, 0, 0.45);
}

.adui-spin-fullscreen {
    position: fixed;
    inset: 0;
    z-index: 9999;
    background: rgba(255, 255, 255, 0.65);
}
"#
    };
}

macro_rules! adui_progress_style {
    () => {
        r#"
.adui-progress {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    font-size: var(--adui-font-size-sm, 13px);
}

.adui-progress-outer {
    flex: 1;
}

.adui-progress-inner {
    width: 100%;
    background: rgba(0, 0, 0, 0.04);
    border-radius: 100px;
    overflow: hidden;
}

.adui-theme-scope[theme-mode="dark"] .adui-progress-inner {
    background: rgba(255, 255, 255, 0.16);
}

.adui-progress-bg {
    background: var(--adui-color-primary, #1677ff);
    border-radius: inherit;
    transition: width var(--adui-motion-duration-mid, 0.24s) ease;
}

.adui-progress-text {
    min-width: 40px;
    text-align: right;
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
}

.adui-progress-status-success .adui-progress-bg {
    background: var(--adui-color-success, #52c41a);
}

.adui-progress-status-exception .adui-progress-bg {
    background: var(--adui-color-error, #ff4d4f);
}

.adui-progress-status-active .adui-progress-bg {
    background: var(--adui-color-primary, #1677ff);
}

.adui-progress-circle {
    flex-direction: column;
}

.adui-progress-circle-inner {
    box-sizing: border-box;
    border-style: solid;
    border-color: transparent;
    border-radius: 50%;
}

.adui-progress-circle .adui-progress-text {
    margin-top: 8px;
    text-align: center;
}
"#
    };
}

macro_rules! adui_statistic_style {
    () => {
        r#"
.adui-statistic {
    display: inline-flex;
    flex-direction: column;
}

.adui-statistic-title {
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
    font-size: var(--adui-font-size-sm, 13px);
    margin-bottom: 4px;
}

.adui-statistic-content {
    display: inline-flex;
    align-items: baseline;
    gap: 4px;
}

.adui-statistic-prefix,
.adui-statistic-suffix {
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
}

.adui-statistic-value {
    font-size: 20px;
    font-weight: 600;
    color: var(--adui-color-text);
}
"#
    };
}

macro_rules! adui_skeleton_style {
    () => {
        r#"
.adui-skeleton {
    display: block;
    padding: 12px 16px;
    background: var(--adui-color-bg-container);
    border-radius: var(--adui-radius);
}

.adui-skeleton-title,
.adui-skeleton-paragraph-line {
    height: 14px;
    margin-bottom: 8px;
    border-radius: 999px;
    background: rgba(0, 0, 0, 0.06);
}

.adui-theme-scope[theme-mode="dark"] .adui-skeleton-title,
.adui-theme-scope[theme-mode="dark"] .adui-skeleton-paragraph-line {
    background: rgba(255, 255, 255, 0.1);
}

.adui-skeleton-title {
    width: 40%;
}

.adui-skeleton-paragraph {
    margin-top: 8px;
}

.adui-skeleton-paragraph-line-last {
    width: 60%;
}

.adui-skeleton-active .adui-skeleton-title,
.adui-skeleton-active .adui-skeleton-paragraph-line {
    position: relative;
    overflow: hidden;
}

.adui-skeleton-active .adui-skeleton-title::after,
.adui-skeleton-active .adui-skeleton-paragraph-line::after {
    content: "";
    position: absolute;
    inset: 0;
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.35), transparent);
    animation: adui-skeleton-shimmer 1.2s ease-in-out infinite;
}

@keyframes adui-skeleton-shimmer {
    0% { transform: translateX(-100%); }
    100% { transform: translateX(100%); }
}
"#
    };
}

macro_rules! adui_list_style {
    () => {
        r#"
.adui-list {
    background: var(--adui-color-bg-container);
    border-radius: var(--adui-radius);
    border: 1px solid transparent;
    padding: 0;
    margin: 0;
}

.adui-list-bordered {
    border-color: var(--adui-color-border);
}

.adui-list-sm .adui-list-item {
    padding: 8px 12px;
}

.adui-list-lg .adui-list-item {
    padding: 16px 20px;
}

.adui-list-header,
.adui-list-footer {
    padding: 12px 16px;
    border-bottom: 1px solid var(--adui-color-border);
}

.adui-list-footer {
    border-top: 1px solid var(--adui-color-border);
    border-bottom: none;
}

.adui-list-body {
    padding: 0;
}

.adui-list-items {
    list-style: none;
    margin: 0;
    padding: 0;
}

.adui-list-item {
    padding: 12px 16px;
    border-bottom: 1px solid var(--adui-color-border);
}

.adui-list-item:last-child {
    border-bottom: none;
}

.adui-list-empty {
    padding: 16px;
}

.adui-list-pagination {
    padding: 8px 16px;
    text-align: right;
}
"#
    };
}

macro_rules! adui_tabs_style {
    () => {
        r#"
.adui-tabs {
    margin: 0;
}

.adui-tabs-nav {
    display: flex;
    border-bottom: 1px solid var(--adui-color-border);
    margin-bottom: 8px;
}

.adui-tabs-nav-list {
    display: flex;
    gap: 16px;
}

.adui-tabs-tab {
    position: relative;
    padding: 8px 0;
    margin: 0;
    border: none;
    background: transparent;
    cursor: pointer;
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
    font-size: var(--adui-font-size, 14px);
}

.adui-tabs-tab:focus-visible {
    outline: none;
}

.adui-tabs-tab-active {
    color: var(--adui-color-text);
    font-weight: 500;
}

.adui-tabs-tab-active::after {
    content: "";
    position: absolute;
    left: 0;
    right: 0;
    bottom: -1px;
    height: 2px;
    background: var(--adui-color-primary);
    border-radius: 2px 2px 0 0;
}

.adui-tabs-tab-disabled {
    cursor: not-allowed;
    color: var(--adui-color-text-disabled);
}

.adui-tabs-sm .adui-tabs-tab {
    padding: 4px 0;
    font-size: var(--adui-font-size-sm, 13px);
}

.adui-tabs-lg .adui-tabs-tab {
    padding: 10px 0;
    font-size: var(--adui-font-size-lg, 16px);
}

.adui-tabs-content {
    padding-top: 4px;
}

.adui-tabs-tabpane {
    padding-top: 4px;
}
"#
    };
}

macro_rules! adui_card_style {
    () => {
        r#"
.adui-card {
    position: relative;
    border-radius: var(--adui-radius);
    background: var(--adui-color-bg-container);
    border: 1px solid transparent;
    box-shadow: none;
    transition: box-shadow var(--adui-motion-duration-fast, 0.16s) ease,
                transform var(--adui-motion-duration-fast, 0.16s) ease,
                border-color var(--adui-motion-duration-fast, 0.16s) ease;
}

.adui-card-bordered {
    border-color: var(--adui-color-border);
}

.adui-card-hoverable {
    cursor: pointer;
}

.adui-card-hoverable:hover {
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.08);
    transform: translateY(-1px);
}

.adui-card-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    border-bottom: 1px solid var(--adui-color-border);
}

.adui-card-head-title {
    font-weight: 500;
}

.adui-card-head-extra {
    margin-left: 12px;
}

.adui-card-body {
    padding: 16px;
}

.adui-card-sm .adui-card-head {
    padding: 8px 12px;
}

.adui-card-sm .adui-card-body {
    padding: 12px;
}

.adui-card-lg .adui-card-head {
    padding: 16px 20px;
}

.adui-card-lg .adui-card-body {
    padding: 20px;
}
"#
    };
}

macro_rules! adui_tag_style {
    () => {
        r#"
.adui-tag {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 0 7px;
    min-height: 22px;
    border-radius: 2px;
    font-size: 12px;
    line-height: 1.6;
    border: 1px solid transparent;
    background: rgba(0, 0, 0, 0.02);
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
}

.adui-tag-default {
    border-color: var(--adui-color-border);
}

.adui-tag-primary {
    border-color: var(--adui-color-primary);
    color: var(--adui-color-primary);
    background: rgba(22, 119, 255, 0.08);
}

.adui-tag-success {
    border-color: var(--adui-color-success);
    color: var(--adui-color-success);
    background: rgba(82, 196, 26, 0.08);
}

.adui-tag-warning {
    border-color: var(--adui-color-warning);
    color: var(--adui-color-warning);
    background: rgba(250, 173, 20, 0.08);
}

.adui-tag-error {
    border-color: var(--adui-color-error);
    color: var(--adui-color-error);
    background: rgba(255, 77, 79, 0.08);
}

.adui-tag-checkable {
    cursor: pointer;
}

.adui-tag-checkable-checked {
    background: var(--adui-color-primary);
    color: #fff;
    border-color: transparent;
}

.adui-tag-close {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    border: none;
    background: transparent;
    cursor: pointer;
    color: inherit;
}

.adui-tag-close:focus-visible {
    outline: none;
}
"#
    };
}

macro_rules! adui_badge_style {
    () => {
        r#"
.adui-badge {
    position: relative;
    display: inline-block;
}

.adui-badge > *:not(.adui-badge-count):not(.adui-badge-dot) {
    vertical-align: middle;
}

.adui-badge-count,
.adui-badge-dot {
    position: absolute;
    top: 0;
    right: 0;
    transform: translate(50%, -50%);
}

.adui-badge-count {
    min-width: 20px;
    height: 20px;
    padding: 0 6px;
    border-radius: 999px;
    background: var(--adui-color-error);
    color: #fff;
    font-size: 12px;
    line-height: 20px;
    text-align: center;
    box-sizing: border-box;
}

.adui-badge-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--adui-color-error);
}

.adui-badge-status-success .adui-badge-count,
.adui-badge-status-success .adui-badge-dot {
    background: var(--adui-color-success);
}

.adui-badge-status-warning .adui-badge-count,
.adui-badge-status-warning .adui-badge-dot {
    background: var(--adui-color-warning);
}

.adui-badge-status-error .adui-badge-count,
.adui-badge-status-error .adui-badge-dot {
    background: var(--adui-color-error);
}
"#
    };
}

macro_rules! adui_avatar_style {
    () => {
        r#"
.adui-avatar {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    background: var(--adui-color-bg-container);
    color: var(--adui-color-text);
    overflow: hidden;
}

.adui-avatar-circle {
    border-radius: 50%;
}

.adui-avatar-square {
    border-radius: var(--adui-radius);
}

.adui-avatar-sm {
    width: 24px;
    height: 24px;
    font-size: 12px;
}

.adui-avatar-md {
    width: 32px;
    height: 32px;
    font-size: 14px;
}

.adui-avatar-lg {
    width: 40px;
    height: 40px;
    font-size: 16px;
}

.adui-avatar-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
}

.adui-avatar-icon,
.adui-avatar-text {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
}

.adui-avatar-group {
    display: inline-flex;
}

.adui-avatar-group .adui-avatar {
    margin-left: -8px;
    border: 2px solid var(--adui-color-bg-base);
}

.adui-avatar-group .adui-avatar:first-child {
    margin-left: 0;
}
"#
    };
}

macro_rules! adui_table_style {
    () => {
        r#"
.adui-table {
    width: 100%;
    background: var(--adui-color-bg-container);
    border-radius: var(--adui-radius);
    border: 1px solid transparent;
    overflow: hidden;
}

.adui-table-bordered {
    border-color: var(--adui-color-border);
}

.adui-table-header {
    border-bottom: 1px solid var(--adui-color-border);
}

.adui-table-row {
    display: flex;
}

.adui-table-row-header {
    background: rgba(0, 0, 0, 0.02);
    font-weight: 500;
}

.adui-table-cell {
    flex: 1;
    padding: 8px 12px;
    border-right: 1px solid var(--adui-color-border);
    box-sizing: border-box;
}

.adui-table-row .adui-table-cell:last-child {
    border-right: none;
}

.adui-table-body-inner .adui-table-row:nth-child(even) {
    background: rgba(0, 0, 0, 0.01);
}

.adui-table-align-left { text-align: left; }
.adui-table-align-center { text-align: center; }
.adui-table-align-right { text-align: right; }

.adui-table-empty {
    padding: 16px;
}

.adui-table-pagination {
    padding: 8px 16px;
    text-align: right;
}
"#
    };
}

macro_rules! adui_masonry_style {
    () => {
        r#"
.adui-masonry > * {
    break-inside: avoid;
    margin-bottom: var(--adui-masonry-row-gap, var(--adui-masonry-gap, 16px));
}
"#
    };
}

macro_rules! adui_splitter_style {
    () => {
        r#"
.adui-splitter {
    border: 1px solid var(--adui-color-border);
    border-radius: var(--adui-radius);
    padding: 8px;
    background: var(--adui-color-bg-container);
}

.adui-splitter-pane {
    background: var(--adui-color-bg-container);
    border: 1px solid var(--adui-color-border);
    border-radius: var(--adui-radius);
    padding: 12px;
    min-height: 60px;
}

.adui-splitter-gutter {
    flex: 0 0 6px;
    background: rgba(0,0,0,0.02);
}
.adui-splitter-horizontal .adui-splitter-gutter {
    cursor: col-resize;
}
.adui-splitter-vertical .adui-splitter-gutter {
    cursor: row-resize;
}
"#
    };
}

macro_rules! adui_float_button_style {
    () => {
        r#"
.adui-float-btn {
    position: relative;
    display: inline-flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 6px;
    min-width: var(--adui-fb-size, 56px);
    height: var(--adui-fb-size, 56px);
    padding: 0 var(--adui-fb-padding-inline, 0px);
    border: 1px solid var(--adui-fb-border);
    background: var(--adui-fb-bg);
    color: var(--adui-fb-color);
    border-radius: var(--adui-fb-radius);
    box-shadow: var(--adui-fb-shadow);
    cursor: pointer;
    transition: all var(--adui-motion-duration-fast, 0.18s) ease;
    text-decoration: none;
    text-align: center;
}

.adui-float-btn-individual {
    position: fixed;
    z-index: 99;
}

.adui-float-btn:hover {
    background: var(--adui-fb-bg-hover);
    color: var(--adui-fb-color-hover);
    border-color: var(--adui-fb-border-hover);
    transform: translateY(-2px);
}

.adui-float-btn:focus-visible {
    outline: none;
    box-shadow: 0 0 0 3px rgba(64, 150, 255, 0.35), var(--adui-fb-shadow);
}

.adui-float-btn:active {
    background: var(--adui-fb-bg-active);
    color: var(--adui-fb-color-active);
    border-color: var(--adui-fb-border-active);
    transform: translateY(0);
}

.adui-float-btn-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 18px;
    line-height: 1;
}

.adui-float-btn-icon-only .adui-float-btn-icon {
    font-size: 22px;
}

.adui-float-btn-circle {
    padding: 0;
}

.adui-float-btn-square {
    padding-inline: var(--adui-fb-padding-inline, 12px);
}

.adui-float-btn-square .adui-float-btn-content {
    margin-top: 4px;
}

.adui-float-btn-content {
    font-size: 12px;
    line-height: 1.2;
    text-align: center;
    white-space: normal;
    word-break: break-word;
}

.adui-float-btn-badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 18px;
    height: 18px;
    padding: 0 6px;
    border-radius: 999px;
    background: var(--adui-color-error);
    color: #fff;
    font-size: 11px;
    position: absolute;
    top: 0;
    right: 0;
    transform: translate(50%, -50%);
}

.adui-float-btn-badge-dot {
    width: 8px;
    height: 8px;
    padding: 0;
    border-radius: 50%;
}

[dir="rtl"] .adui-float-btn-badge:not(.adui-float-btn-badge-dot) {
    right: auto;
    left: 0;
    transform: translate(-50%, -50%);
}

.adui-float-btn-group {
    position: fixed;
    display: flex;
    flex-direction: column;
    gap: var(--adui-fb-group-gap, 12px);
}

.adui-float-btn-group > .adui-float-btn {
    position: relative;
}

.adui-float-btn-group-pure {
    position: static;
}

	.adui-float-btn-primary {
	    color: #fff;
	}
	"#
    };
}

macro_rules! adui_tooltip_style {
    () => {
        r#"
.adui-tooltip-root {
    position: relative;
    display: inline-block;
}

.adui-tooltip {
    padding: 4px 8px;
    background: rgba(0, 0, 0, 0.75);
    color: #fff;
    border-radius: var(--adui-radius-sm, 4px);
    font-size: 12px;
    line-height: 1.4;
    box-shadow: 0 2px 8px rgba(0,0,0,0.15);
    max-width: 320px;
    pointer-events: auto;
}

.adui-tooltip-inner {
    white-space: normal;
    word-break: break-word;
}

.adui-popover-root {
    position: relative;
    display: inline-block;
}

.adui-popover {
    background: var(--adui-color-bg-container);
    color: var(--adui-color-text);
    border-radius: var(--adui-radius-sm, 4px);
    border: 1px solid var(--adui-color-border);
    box-shadow: var(--adui-shadow-secondary);
    min-width: 200px;
    max-width: 360px;
}

.adui-popover-inner {
    padding: 12px 16px;
}

.adui-popover-title {
    margin-bottom: 8px;
    font-weight: 600;
}

.adui-popover-content {
    font-size: 14px;
}

.adui-popconfirm-inner {
    min-width: 220px;
}

.adui-popconfirm-body {
    margin-bottom: 12px;
}

.adui-popconfirm-title {
    font-weight: 600;
    margin-bottom: 4px;
}

.adui-popconfirm-description {
    font-size: 13px;
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
}

.adui-popconfirm-footer {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
}
"#
    };
}

macro_rules! adui_keyframes_style {
    () => {
        r#"
@keyframes adui-spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
}
"#
    };
}

macro_rules! adui_watermark_style {
    () => {
        r#"
.adui-watermark-wrapper {
    position: relative;
    overflow: hidden;
}

.adui-watermark {
    position: absolute;
    inset: 0;
    pointer-events: none;
    background-repeat: repeat;
    z-index: 9;
}
"#
    };
}

macro_rules! adui_qrcode_style {
    () => {
        r#"
.adui-qrcode-wrapper {
    display: inline-block;
}

.adui-qrcode {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 12px;
    border-radius: var(--adui-radius, 8px);
    border: 1px solid var(--adui-color-border, #d9d9d9);
    background: var(--adui-color-bg-container, #ffffff);
}

.adui-qrcode-borderless {
    border: none;
    padding: 0;
}

.adui-qrcode svg {
    display: block;
}

.adui-qrcode-cover {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(255, 255, 255, 0.9);
    border-radius: inherit;
}

.adui-theme-scope[theme-mode="dark"] .adui-qrcode-cover {
    background: rgba(0, 0, 0, 0.75);
}

.adui-qrcode-status {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    text-align: center;
}

.adui-qrcode-status-text {
    font-size: 14px;
    color: var(--adui-color-text-secondary, var(--adui-color-text-muted));
}

.adui-qrcode-status-icon {
    font-size: 24px;
    color: var(--adui-color-success, #52c41a);
}

.adui-qrcode-refresh-btn {
    padding: 4px 12px;
    border: 1px solid var(--adui-color-primary, #1677ff);
    border-radius: var(--adui-radius-sm, 4px);
    background: transparent;
    color: var(--adui-color-primary, #1677ff);
    cursor: pointer;
    font-size: 14px;
    transition: all var(--adui-motion-duration-fast, 0.16s) ease;
}

.adui-qrcode-refresh-btn:hover {
    background: var(--adui-color-primary, #1677ff);
    color: #fff;
}

.adui-qrcode-spinner {
    width: 24px;
    height: 24px;
    border: 2px solid var(--adui-color-primary, #1677ff);
    border-right-color: transparent;
    border-radius: 50%;
    animation: adui-spin 0.9s linear infinite;
}

.adui-qrcode-expired .adui-qrcode-cover {
    background: rgba(255, 255, 255, 0.96);
}

.adui-theme-scope[theme-mode="dark"] .adui-qrcode-expired .adui-qrcode-cover {
    background: rgba(0, 0, 0, 0.85);
}
"#
    };
}

pub const SCOPE_STYLE: &str = adui_scope_style!();
pub const BUTTON_STYLE: &str = adui_button_style!();
pub const ICON_STYLE: &str = adui_icon_style!();
pub const DIVIDER_STYLE: &str = adui_divider_style!();
pub const TYPOGRAPHY_STYLE: &str = adui_typography_style!();
pub const LAYOUT_STYLE: &str = adui_layout_style!();
pub const GRID_STYLE: &str = adui_grid_style!();
pub const SPACE_STYLE: &str = adui_space_style!();
pub const FLEX_STYLE: &str = adui_flex_style!();
pub const FORM_STYLE: &str = adui_form_style!();
pub const CONTROL_STYLE: &str = adui_control_style!();
pub const MASONRY_STYLE: &str = adui_masonry_style!();
pub const SPLITTER_STYLE: &str = adui_splitter_style!();
pub const FLOAT_BUTTON_STYLE: &str = adui_float_button_style!();
pub const TOOLTIP_STYLE: &str = adui_tooltip_style!();
pub const KEYFRAMES_STYLE: &str = adui_keyframes_style!();
pub const EMPTY_STYLE: &str = adui_empty_style!();
pub const SPIN_STYLE: &str = adui_spin_style!();
pub const PROGRESS_STYLE: &str = adui_progress_style!();
pub const STATISTIC_STYLE: &str = adui_statistic_style!();
pub const SKELETON_STYLE: &str = adui_skeleton_style!();
pub const LIST_STYLE: &str = adui_list_style!();
pub const TABS_STYLE: &str = adui_tabs_style!();
pub const CARD_STYLE: &str = adui_card_style!();
pub const TAG_STYLE: &str = adui_tag_style!();
pub const BADGE_STYLE: &str = adui_badge_style!();
pub const AVATAR_STYLE: &str = adui_avatar_style!();
pub const TABLE_STYLE: &str = adui_table_style!();
pub const RESULT_STYLE: &str = adui_result_style!();
pub const STEPS_STYLE: &str = adui_steps_style!();
pub const WATERMARK_STYLE: &str = adui_watermark_style!();
pub const QRCODE_STYLE: &str = adui_qrcode_style!();

pub const THEME_BASE_STYLE: &str = concat!(
    adui_scope_style!(),
    adui_button_style!(),
    adui_icon_style!(),
    adui_divider_style!(),
    adui_alert_style!(),
    adui_result_style!(),
    adui_calendar_style!(),
    adui_date_picker_style!(),
    adui_time_picker_style!(),
    adui_steps_style!(),
    adui_typography_style!(),
    adui_layout_style!(),
    adui_grid_style!(),
    adui_flex_style!(),
    adui_form_style!(),
    adui_control_style!(),
    adui_tooltip_style!(),
    adui_space_style!(),
    adui_masonry_style!(),
    adui_splitter_style!(),
    adui_float_button_style!(),
    adui_keyframes_style!(),
    adui_empty_style!(),
    adui_spin_style!(),
    adui_progress_style!(),
    adui_statistic_style!(),
    adui_skeleton_style!(),
    adui_list_style!(),
    adui_tabs_style!(),
    adui_card_style!(),
    adui_tag_style!(),
    adui_badge_style!(),
    adui_avatar_style!(),
    adui_table_style!(),
    adui_watermark_style!(),
    adui_qrcode_style!(),
);

/// Theme mode tracks the seed variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
    Custom,
}

/// Core design tokens needed by early components.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThemeTokens {
    pub color_primary: String,
    pub color_primary_hover: String,
    pub color_primary_active: String,
    pub color_success: String,
    pub color_success_hover: String,
    pub color_success_active: String,
    pub color_warning: String,
    pub color_warning_hover: String,
    pub color_warning_active: String,
    pub color_error: String,
    pub color_error_hover: String,
    pub color_error_active: String,
    pub color_link: String,
    pub color_link_hover: String,
    pub color_link_active: String,
    pub color_text: String,
    pub color_text_muted: String,
    pub color_text_secondary: String,
    pub color_text_disabled: String,
    pub color_split: String,
    pub color_bg_base: String,
    pub color_bg_container: String,
    pub color_bg_layout: String,
    pub color_border: String,
    pub color_border_hover: String,
    pub border_radius: f32,
    pub border_radius_sm: f32,
    pub border_radius_lg: f32,
    pub control_height: f32,
    pub control_height_small: f32,
    pub control_height_large: f32,
    pub padding_inline: f32,
    pub padding_inline_small: f32,
    pub padding_inline_large: f32,
    pub padding_block: f32,
    pub padding_block_small: f32,
    pub padding_block_large: f32,
    pub font_size: f32,
    pub font_size_small: f32,
    pub font_size_large: f32,
    pub line_height: f32,
    pub control_line_width: f32,
    pub motion_duration_fast: f32,
    pub motion_duration_mid: f32,
    pub shadow: String,
    pub shadow_secondary: String,
}

impl ThemeTokens {
    /// Ant Design 6.x inspired light tokens.
    pub fn light() -> Self {
        Self {
            color_primary: "#1677ff".into(),
            color_primary_hover: "#4096ff".into(),
            color_primary_active: "#0958d9".into(),
            color_success: "#52c41a".into(),
            color_success_hover: "#73d13d".into(),
            color_success_active: "#389e0d".into(),
            color_warning: "#faad14".into(),
            color_warning_hover: "#ffc53d".into(),
            color_warning_active: "#d48806".into(),
            color_error: "#ff4d4f".into(),
            color_error_hover: "#ff7875".into(),
            color_error_active: "#d9363e".into(),
            color_link: "#1677ff".into(),
            color_link_hover: "#4096ff".into(),
            color_link_active: "#0958d9".into(),
            color_text: "#1f1f1f".into(),
            color_text_muted: "#595959".into(),
            color_text_secondary: "#8c8c8c".into(),
            color_text_disabled: "rgba(0,0,0,0.25)".into(),
            color_split: "#f0f0f0".into(),
            color_bg_base: "#f5f5f5".into(),
            color_bg_container: "#ffffff".into(),
            color_bg_layout: "#f5f5f5".into(),
            color_border: "#d9d9d9".into(),
            color_border_hover: "#91caff".into(),
            border_radius: 6.0,
            border_radius_sm: 4.0,
            border_radius_lg: 8.0,
            control_height: 32.0,
            control_height_small: 24.0,
            control_height_large: 40.0,
            padding_inline: 15.0,
            padding_inline_small: 12.0,
            padding_inline_large: 18.0,
            padding_block: 6.0,
            padding_block_small: 4.0,
            padding_block_large: 8.0,
            font_size: 14.0,
            font_size_small: 13.0,
            font_size_large: 16.0,
            line_height: 1.5715,
            control_line_width: 1.0,
            motion_duration_fast: 0.16,
            motion_duration_mid: 0.24,
            shadow: "0 2px 0 rgba(5, 145, 255, 0.1)".into(),
            shadow_secondary: "0 6px 16px rgba(0,0,0,0.08)".into(),
        }
    }

    /// Ant Design 6.x inspired dark tokens.
    pub fn dark() -> Self {
        Self {
            color_primary: "#177ddc".into(),
            color_primary_hover: "#3c9ae8".into(),
            color_primary_active: "#1668b2".into(),
            color_success: "#49aa19".into(),
            color_success_hover: "#6abe39".into(),
            color_success_active: "#3f8618".into(),
            color_warning: "#d89614".into(),
            color_warning_hover: "#e8b339".into(),
            color_warning_active: "#ad6800".into(),
            color_error: "#f16364".into(),
            color_error_hover: "#ff7875".into(),
            color_error_active: "#d84a45".into(),
            color_link: "#3c9ae8".into(),
            color_link_hover: "#65b7f3".into(),
            color_link_active: "#2b74b1".into(),
            color_text: "#f0f0f0".into(),
            color_text_muted: "#bfbfbf".into(),
            color_text_secondary: "#8c8c8c".into(),
            color_text_disabled: "rgba(255,255,255,0.35)".into(),
            color_split: "#303030".into(),
            color_bg_base: "#141414".into(),
            color_bg_container: "#1f1f1f".into(),
            color_bg_layout: "#0f0f0f".into(),
            color_border: "#2a2a2a".into(),
            color_border_hover: "#3a3a3a".into(),
            border_radius: 6.0,
            border_radius_sm: 4.0,
            border_radius_lg: 8.0,
            control_height: 32.0,
            control_height_small: 24.0,
            control_height_large: 40.0,
            padding_inline: 15.0,
            padding_inline_small: 12.0,
            padding_inline_large: 18.0,
            padding_block: 6.0,
            padding_block_small: 4.0,
            padding_block_large: 8.0,
            font_size: 14.0,
            font_size_small: 13.0,
            font_size_large: 16.0,
            line_height: 1.5715,
            control_line_width: 1.0,
            motion_duration_fast: 0.16,
            motion_duration_mid: 0.24,
            shadow: "0 2px 0 rgba(23, 125, 220, 0.25)".into(),
            shadow_secondary: "0 6px 16px rgba(0,0,0,0.5)".into(),
        }
    }
}

/// Theme object bundling mode and resolved tokens.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub mode: ThemeMode,
    pub tokens: ThemeTokens,
}

impl Theme {
    /// Build a theme from a given mode using defaults.
    pub fn for_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Light => Self::light(),
            ThemeMode::Dark => Self::dark(),
            ThemeMode::Custom => Self {
                mode,
                tokens: ThemeTokens::light(),
            },
        }
    }

    /// Light preset.
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            tokens: ThemeTokens::light(),
        }
    }

    /// Dark preset.
    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            tokens: ThemeTokens::dark(),
        }
    }
}

/// Handle for reading or mutating the active theme from components.
#[derive(Clone, Copy)]
pub struct ThemeHandle {
    signal: Signal<Theme>,
}

impl ThemeHandle {
    pub fn theme(&self) -> Theme {
        self.signal.read().clone()
    }

    pub fn tokens(&self) -> ThemeTokens {
        self.signal.read().tokens.clone()
    }

    pub fn set_theme(&self, theme: Theme) {
        let mut signal = self.signal;
        signal.set(theme);
    }

    pub fn set_mode(&self, mode: ThemeMode) {
        let mut signal = self.signal;
        signal.set(Theme::for_mode(mode));
    }

    pub fn update_tokens(&self, mode: Option<ThemeMode>, update: impl FnOnce(&mut ThemeTokens)) {
        let mut signal = self.signal;
        signal.with_mut(|theme| {
            if let Some(next_mode) = mode {
                theme.mode = next_mode;
                theme.tokens = match next_mode {
                    ThemeMode::Light => ThemeTokens::light(),
                    ThemeMode::Dark => ThemeTokens::dark(),
                    ThemeMode::Custom => theme.tokens.clone(),
                };
            }
            update(&mut theme.tokens);
        });
    }
}

/// Hook to access the current theme signal.
pub fn use_theme() -> ThemeHandle {
    let signal: Signal<Theme> = use_context();
    ThemeHandle { signal }
}

/// Props for [`ThemeProvider`].
#[derive(Props, Clone, PartialEq)]
pub struct ThemeProviderProps {
    #[props(optional)]
    pub theme: Option<Theme>,
    pub children: Element,
}

/// Provide theme context and CSS variables for descendant components.
#[component]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
    let initial = props.theme.clone().unwrap_or_else(Theme::light);
    let signal = use_context_provider(|| Signal::new(initial));
    let handle = ThemeHandle { signal };
    let tokens = handle.tokens();
    let css_vars = tokens_to_css_vars(&tokens);

    rsx! {
        style { {THEME_BASE_STYLE} }
        div {
            class: "adui-theme-scope",
            style: css_vars,
            {props.children}
        }
    }
}

fn tokens_to_css_vars(tokens: &ThemeTokens) -> String {
    format!(
        "--adui-color-primary:{};\
        --adui-color-primary-hover:{};\
        --adui-color-primary-active:{};\
        --adui-color-success:{};\
        --adui-color-success-hover:{};\
        --adui-color-success-active:{};\
        --adui-color-warning:{};\
        --adui-color-warning-hover:{};\
        --adui-color-warning-active:{};\
        --adui-color-error:{};\
        --adui-color-error-hover:{};\
        --adui-color-error-active:{};\
        --adui-color-link:{};\
        --adui-color-link-hover:{};\
        --adui-color-link-active:{};\
        --adui-color-text:{};\
        --adui-color-text-muted:{};\
        --adui-color-text-secondary:{};\
        --adui-color-text-disabled:{};\
        --adui-color-split:{};\
        --adui-color-bg-base:{};\
        --adui-color-bg-container:{};\
        --adui-color-bg-layout:{};\
        --adui-color-border:{};\
        --adui-color-border-hover:{};\
        --adui-radius:{}px;\
        --adui-radius-sm:{}px;\
        --adui-radius-lg:{}px;\
        --adui-control-line-width:{}px;\
        --adui-control-height:{}px;\
        --adui-control-height-sm:{}px;\
        --adui-control-height-lg:{}px;\
        --adui-padding-inline:{}px;\
        --adui-padding-inline-sm:{}px;\
        --adui-padding-inline-lg:{}px;\
        --adui-padding-block:{}px;\
        --adui-padding-block-sm:{}px;\
        --adui-padding-block-lg:{}px;\
        --adui-font-size:{}px;\
        --adui-font-size-sm:{}px;\
        --adui-font-size-lg:{}px;\
        --adui-line-height:{};\
        --adui-motion-duration-fast:{}s;\
        --adui-motion-duration-mid:{}s;\
        --adui-shadow:{};\
        --adui-shadow-secondary:{};",
        tokens.color_primary,
        tokens.color_primary_hover,
        tokens.color_primary_active,
        tokens.color_success,
        tokens.color_success_hover,
        tokens.color_success_active,
        tokens.color_warning,
        tokens.color_warning_hover,
        tokens.color_warning_active,
        tokens.color_error,
        tokens.color_error_hover,
        tokens.color_error_active,
        tokens.color_link,
        tokens.color_link_hover,
        tokens.color_link_active,
        tokens.color_text,
        tokens.color_text_muted,
        tokens.color_text_secondary,
        tokens.color_text_disabled,
        tokens.color_split,
        tokens.color_bg_base,
        tokens.color_bg_container,
        tokens.color_bg_layout,
        tokens.color_border,
        tokens.color_border_hover,
        tokens.border_radius,
        tokens.border_radius_sm,
        tokens.border_radius_lg,
        tokens.control_line_width,
        tokens.control_height,
        tokens.control_height_small,
        tokens.control_height_large,
        tokens.padding_inline,
        tokens.padding_inline_small,
        tokens.padding_inline_large,
        tokens.padding_block,
        tokens.padding_block_small,
        tokens.padding_block_large,
        tokens.font_size,
        tokens.font_size_small,
        tokens.font_size_large,
        tokens.line_height,
        tokens.motion_duration_fast,
        tokens.motion_duration_mid,
        tokens.shadow,
        tokens.shadow_secondary,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_to_css_vars_emits_expected_variables() {
        let tokens = ThemeTokens::light();
        let css = tokens_to_css_vars(&tokens);
        assert!(
            css.contains("--adui-color-primary:#1677ff;"),
            "primary color missing: {css}"
        );
        assert!(css.contains("--adui-color-bg-container:#ffffff;"));
        assert!(css.contains("--adui-font-size:14px;"));
        assert!(css.contains("--adui-shadow-secondary:0 6px 16px rgba(0,0,0,0.08);"));
    }

    #[test]
    fn tokens_to_css_vars_reflect_custom_updates() {
        let mut tokens = ThemeTokens::light();
        tokens.color_primary = "#000000".into();
        tokens.font_size = 18.0;
        let css = tokens_to_css_vars(&tokens);
        assert!(css.contains("--adui-color-primary:#000000;"));
        assert!(css.contains("--adui-font-size:18px;"));
    }
}
