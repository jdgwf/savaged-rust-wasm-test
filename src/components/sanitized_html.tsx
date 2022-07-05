import * as React from 'react';
import * as SanitizeHTML from 'sanitize-html';

export default class SanitizedHTML extends React.Component<ISanitizedHTMLProps, ISanitizedHTMLState> {

    constructor(props: ISanitizedHTMLProps) {
        super(props);
        this.state = {
            messageText: "To update your password, just type it twice in the fields above.",
            messageClass: "alert alert-info",
        }
    }

    // For the SanitizeHTML function/module docs go here:
    // https://www.npmjs.com/package/sanitize-html

    render = (): React.ReactElement => {
        if( this.props.raw ) {
            return (
                <span
                    //@ts-ignore
                    dangerouslySetInnerHTML={
                        {
                            __html: this.props.html
                        }
                    }
                ></span>
            )
        } else {
            return (
                <span
                    dangerouslySetInnerHTML={
                        {
                            //@ts-ignore
                            __html: SanitizeHTML(
                                this.props.html,
                                {
                                    allowedTags: [
                                        "h1",
                                        "h2",
                                        "h3",
                                        "h4",
                                        "h5",
                                        "a",

                                        "div",
                                        "span",
                                        "p",

                                        "blockquote",

                                        "table",
                                        "tr",
                                        "th",
                                        "td",
                                        "thead",
                                        "tbody",

                                        "ol",
                                        "ul",
                                        "li",

                                        "img",
                                        "code",

                                        "strong",
                                        "em",

                                        "br",
                                        "hr"
                                    ],
                                    allowedAttributes: {
                                        'td': [ 'class', 'colspan', 'rowspan', ],
                                        'th': [ 'class', 'colspan', 'rowspan', ],
                                        'table': [ 'class', ],
                                        'tr': [ 'class', ],

                                        'a': [ 'class', 'title', 'href', 'target', ],
                                        'div': [ 'class', 'title', 'id' ],
                                        'span': [ 'class', 'title', 'id' ],
                                        'p': [ 'class', 'id' ],
                                        'ul': [ 'class', 'id' ],
                                        'li': [ 'class', 'id' ],
                                        'img': ['src', 'class', 'id', 'title', 'style'],
                                        'h1': [ 'class', 'id' ],
                                        'h2': [ 'class', 'id' ],
                                        'h3': [ 'class', 'id' ],
                                        'h4': [ 'class', 'id' ],
                                        'h5': [ 'class', 'id' ],
                                    },
                                }
                            )
                        }
                    }
                />
            )
        }
    }
}

interface ISanitizedHTMLProps {
    html: string;
    raw?: boolean;
}

interface ISanitizedHTMLState {

}