(function() {
    'use strict';
    
    htmx.defineExtension('client-side-templates', {
        transformResponse: function(text, xhr, elt) {
            const template = this.findTemplate(elt);
            if (template) {
                try {
                    const data = JSON.parse(text);
                    return this.renderTemplate(template, data, elt);
                } catch (e) {
                    console.warn('client-side-templates: Could not parse JSON response', e);
                    return text;
                }
            }
            return text;
        },
        
        findTemplate: function(elt) {
            const templateTypes = ['mustache', 'handlebars', 'nunjucks', 'xslt'];
            for (const type of templateTypes) {
                const templateId = elt.getAttribute(`${type}-template`);
                const arrayTemplateId = elt.getAttribute(`${type}-array-template`);
                
                if (templateId) {
                    return { type, id: templateId, isArray: false };
                }
                if (arrayTemplateId) {
                    return { type, id: arrayTemplateId, isArray: true };
                }
            }
            return null;
        },
        
        renderTemplate: function(template, data, elt) {
            switch (template.type) {
                case 'mustache':
                    return this.renderMustache(template.id, data, template.isArray);
                case 'handlebars':
                    return this.renderHandlebars(template.id, data, template.isArray);
                case 'nunjucks':
                    return this.renderNunjucks(template.id, data, template.isArray);
                case 'xslt':
                    return this.renderXSLT(template.id, data);
                default:
                    return JSON.stringify(data);
            }
        },
        
        renderMustache: function(templateId, data, isArray) {
            if (typeof Mustache === 'undefined') {
                console.error('Mustache is not loaded');
                return JSON.stringify(data);
            }
            
            const templateElement = document.getElementById(templateId);
            if (!templateElement) {
                console.error(`Template with id '${templateId}' not found`);
                return JSON.stringify(data);
            }
            
            const template = templateElement.innerHTML;
            
            if (isArray && Array.isArray(data)) {
                return data.map(item => Mustache.render(template, item)).join('');
            } else {
                return Mustache.render(template, data);
            }
        },
        
        renderHandlebars: function(templateId, data, isArray) {
            if (typeof Handlebars === 'undefined') {
                console.error('Handlebars is not loaded');
                return JSON.stringify(data);
            }
            
            const templateElement = document.getElementById(templateId);
            if (!templateElement) {
                console.error(`Template with id '${templateId}' not found`);
                return JSON.stringify(data);
            }
            
            const template = Handlebars.compile(templateElement.innerHTML);
            
            if (isArray && Array.isArray(data)) {
                return data.map(item => template(item)).join('');
            } else {
                return template(data);
            }
        },
        
        renderNunjucks: function(templateId, data, isArray) {
            if (typeof nunjucks === 'undefined') {
                console.error('Nunjucks is not loaded');
                return JSON.stringify(data);
            }
            
            const templateElement = document.getElementById(templateId);
            if (!templateElement) {
                console.error(`Template with id '${templateId}' not found`);
                return JSON.stringify(data);
            }
            
            const template = templateElement.innerHTML;
            
            if (isArray && Array.isArray(data)) {
                return data.map(item => nunjucks.renderString(template, item)).join('');
            } else {
                return nunjucks.renderString(template, data);
            }
        },
        
        renderXSLT: function(templateId, data) {
            const templateElement = document.getElementById(templateId);
            if (!templateElement) {
                console.error(`XSLT template with id '${templateId}' not found`);
                return JSON.stringify(data);
            }
            
            try {
                const parser = new DOMParser();
                const xsltDoc = parser.parseFromString(templateElement.innerHTML, 'application/xml');
                const xmlDoc = parser.parseFromString(data, 'application/xml');
                
                const xsltProcessor = new XSLTProcessor();
                xsltProcessor.importStylesheet(xsltDoc);
                const resultDoc = xsltProcessor.transformToFragment(xmlDoc, document);
                
                const serializer = new XMLSerializer();
                return serializer.serializeToString(resultDoc);
            } catch (e) {
                console.error('XSLT transformation error:', e);
                return data;
            }
        }
    });
})();
