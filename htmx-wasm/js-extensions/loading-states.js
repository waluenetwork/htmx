(function() {
    'use strict';
    
    htmx.defineExtension('loading-states', {
        onEvent: function(name, evt) {
            if (name === 'htmx:beforeRequest') {
                this.handleBeforeRequest(evt);
            } else if (name === 'htmx:beforeOnLoad') {
                this.handleBeforeOnLoad(evt);
            } else if (name === 'htmx:afterRequest') {
                this.handleAfterRequest(evt);
            }
        },
        
        handleBeforeRequest: function(evt) {
            const elt = evt.detail.elt;
            
            if (elt.hasAttribute('data-loading-disable')) {
                this.disableElement(elt);
                elt.setAttribute('data-loading-was-disabled', 'true');
            }
            
            if (elt.hasAttribute('data-loading-class')) {
                const className = elt.getAttribute('data-loading-class');
                elt.classList.add(className);
                elt.setAttribute('data-loading-class-added', className);
            }
            
            if (elt.hasAttribute('data-loading-class-remove')) {
                const className = elt.getAttribute('data-loading-class-remove');
                elt.classList.remove(className);
                elt.setAttribute('data-loading-class-removed', className);
            }
            
            if (elt.hasAttribute('data-loading-aria-busy')) {
                elt.setAttribute('aria-busy', 'true');
                elt.setAttribute('data-loading-aria-busy-added', 'true');
            }
            
            if (elt.hasAttribute('data-loading-target')) {
                const targetSelector = elt.getAttribute('data-loading-target');
                const targets = document.querySelectorAll(targetSelector);
                
                targets.forEach(target => {
                    this.applyLoadingState(target, elt);
                });
            }
            
            if (elt.hasAttribute('data-loading-delay')) {
                const delay = parseInt(elt.getAttribute('data-loading-delay'));
                if (delay > 0) {
                    setTimeout(() => {
                        this.applyDelayedLoadingState(elt);
                    }, delay);
                }
            }
        },
        
        handleBeforeOnLoad: function(evt) {
            this.cleanupLoadingStates(evt.detail.elt);
        },
        
        handleAfterRequest: function(evt) {
            this.cleanupLoadingStates(evt.detail.elt);
        },
        
        applyLoadingState: function(target, sourceElt) {
            if (sourceElt.hasAttribute('data-loading-class')) {
                const className = sourceElt.getAttribute('data-loading-class');
                target.classList.add(className);
                target.setAttribute('data-loading-class-added', className);
            }
            
            if (sourceElt.hasAttribute('data-loading-class-remove')) {
                const className = sourceElt.getAttribute('data-loading-class-remove');
                target.classList.remove(className);
                target.setAttribute('data-loading-class-removed', className);
            }
            
            if (sourceElt.hasAttribute('data-loading-aria-busy')) {
                target.setAttribute('aria-busy', 'true');
                target.setAttribute('data-loading-aria-busy-added', 'true');
            }
        },
        
        applyDelayedLoadingState: function(elt) {
            if (elt.hasAttribute('data-loading-class-delay')) {
                const className = elt.getAttribute('data-loading-class-delay');
                elt.classList.add(className);
                elt.setAttribute('data-loading-class-delay-added', className);
            }
        },
        
        disableElement: function(elt) {
            if (elt.tagName === 'BUTTON' || elt.tagName === 'INPUT') {
                elt.disabled = true;
            } else if (elt.tagName === 'FORM') {
                const inputs = elt.querySelectorAll('input, button, select, textarea');
                inputs.forEach(input => {
                    input.disabled = true;
                    input.setAttribute('data-loading-was-disabled-by-form', 'true');
                });
            }
        },
        
        cleanupLoadingStates: function(elt) {
            if (elt.hasAttribute('data-loading-was-disabled')) {
                if (elt.tagName === 'BUTTON' || elt.tagName === 'INPUT') {
                    elt.disabled = false;
                } else if (elt.tagName === 'FORM') {
                    const inputs = elt.querySelectorAll('[data-loading-was-disabled-by-form]');
                    inputs.forEach(input => {
                        input.disabled = false;
                        input.removeAttribute('data-loading-was-disabled-by-form');
                    });
                }
                elt.removeAttribute('data-loading-was-disabled');
            }
            
            if (elt.hasAttribute('data-loading-class-added')) {
                const className = elt.getAttribute('data-loading-class-added');
                elt.classList.remove(className);
                elt.removeAttribute('data-loading-class-added');
            }
            
            if (elt.hasAttribute('data-loading-class-removed')) {
                const className = elt.getAttribute('data-loading-class-removed');
                elt.classList.add(className);
                elt.removeAttribute('data-loading-class-removed');
            }
            
            if (elt.hasAttribute('data-loading-class-delay-added')) {
                const className = elt.getAttribute('data-loading-class-delay-added');
                elt.classList.remove(className);
                elt.removeAttribute('data-loading-class-delay-added');
            }
            
            if (elt.hasAttribute('data-loading-aria-busy-added')) {
                elt.removeAttribute('aria-busy');
                elt.removeAttribute('data-loading-aria-busy-added');
            }
            
            if (elt.hasAttribute('data-loading-target')) {
                const targetSelector = elt.getAttribute('data-loading-target');
                const targets = document.querySelectorAll(targetSelector);
                
                targets.forEach(target => {
                    this.cleanupLoadingStates(target);
                });
            }
        }
    });
})();
